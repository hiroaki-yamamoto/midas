use ::std::error::Error;
use std::convert::TryFrom;

use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::future::{join, join_all, BoxFuture};
use ::futures::stream::BoxStream;
use ::futures::{FutureExt, StreamExt};
use ::mongodb::bson::{doc, oid::ObjectId, to_document, DateTime};
use ::mongodb::options::{UpdateModifications, UpdateOptions};
use ::mongodb::{Collection, Database};
use ::nats::Connection as NatsCon;
use ::ring::hmac;
use ::serde_qs::to_string as to_qs;
use ::slog::Logger;

use ::entities::{BookTicker, ExecutionResult, Order, OrderInner, OrderOption};
use ::errors::{ObjectNotFound, StatusFailure};
use ::executor::Executor as ExecutorTrait;
use ::keychain::KeyChain;
use ::rpc::entities::Exchanges;
use ::sign::Sign;
use ::types::retry::retry_async;
use ::types::ThreadSafeResult;
use ::writers::DatabaseWriter;

use ::clients::binance::{PubClient, REST_ENDPOINT};
use ::observers::binance::{TradeObserver, TradeObserverTrait};

use super::entities::{
  CancelOrderRequest, OrderRequest, OrderResponse, OrderResponseType,
  OrderType, Side,
};

#[derive(Debug, Clone)]
pub struct Executor {
  keychain: KeyChain,
  broker: NatsCon,
  db: Database,
  log: Logger,
  positions: Collection<OrderResponse<f64, DateTime>>,
}

impl Executor {
  pub async fn new(broker: NatsCon, db: Database, log: Logger) -> Self {
    let keychain = KeyChain::new(broker.clone(), db.clone()).await;
    let positions = db.collection("binance.positions");
    let me = Self {
      broker,
      keychain,
      db,
      log,
      positions,
    };
    me.update_indices(&[
      "orderId",
      "clientOrderId",
      "settlementGid",
      "positionGroupId",
    ])
    .await;
    return me;
  }
}

impl PubClient for Executor {}
impl DatabaseWriter for Executor {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return self.positions.name();
  }
}

#[async_trait]
impl ExecutorTrait for Executor {
  async fn open(
    &mut self,
  ) -> ThreadSafeResult<BoxStream<'_, ThreadSafeResult<BookTicker>>> {
    let stream = try_stream! {
      let observer = TradeObserver::new(
      Some(self.db.clone()),
      self.broker.clone(),
      self.log.clone(),
    )
    .await;
    let mut sub = observer.subscribe().await?;
      while let Some(book_ticker) = sub.next().await {
        yield book_ticker;
      }
    };
    return Ok(Box::pin(stream));
  }
  async fn create_order(
    &mut self,
    api_key_id: ObjectId,
    symbol: String,
    price: Option<f64>,
    budget: f64,
    order_option: Option<OrderOption>,
  ) -> ThreadSafeResult<ObjectId> {
    let pos_gid = ObjectId::new();
    let key = self.keychain.get(Exchanges::Binance, api_key_id).await?;
    let key = key.ok_or(ObjectNotFound::new("API KeyPair".to_string()))?;
    let key = key.inner();
    let order_type = order_option
      .clone()
      .map(|_| price.map(|_| OrderType::Limit).unwrap_or(OrderType::Market))
      .unwrap_or(OrderType::Market);
    let cli = self.get_client(&key.pub_key)?;
    let req_lst: Vec<BoxFuture<Result<(), Box<dyn Error + Send + Sync>>>> =
      order_option
        .map(|o| {
          o.calc_trading_amounts(budget)
            .into_iter()
            .enumerate()
            .map(|(index, tr_amount)| {
              let mut order = OrderRequest::<i64>::new(
                symbol.clone(),
                Side::Buy,
                order_type.clone(),
              );
              if o.iceberg {
                order = order.iceberg_qty(Some(tr_amount));
              } else {
                order = order.quantity(Some(tr_amount));
              }
              if order_type == OrderType::Limit {
                order =
                  order.price(Some(o.calc_order_price(price.unwrap(), index)));
              }
              order =
                order.order_response_type(Some(OrderResponseType::RESULT));
              return order;
            })
            .collect::<Vec<OrderRequest<i64>>>()
        })
        .unwrap_or_else(|| {
          let mut order =
            OrderRequest::<i64>::new(symbol, Side::Buy, order_type.clone())
              .order_response_type(Some(OrderResponseType::RESULT));
          if order_type == OrderType::Limit {
            order = order.price(price);
          }
          return vec![order];
        })
        .into_iter()
        .map(|order| {
          let me = self.clone();
          let cli = cli.clone();
          let pos_gid = pos_gid.clone();
          let key = key.clone();
          async move {
            let qs = to_qs(&order)?;
            let signature = me.sign(qs.clone(), key.prv_key);
            let qs = format!("{}&signature={}", qs, signature);
            return retry_async(5, || async {
              let resp = cli
                .post(format!("{}/api/v3/order?{}", REST_ENDPOINT, qs))
                .send()
                .await;
              let resp: ThreadSafeResult<()> = match resp {
                Ok(resp) => {
                  let payload: OrderResponse<String, i64> = resp.json().await?;
                  let mut payload =
                    OrderResponse::<f64, DateTime>::try_from(payload)?;
                  payload.position_group_id = Some(pos_gid.clone());
                  let _ = me
                    .positions
                    .update_one(
                      doc! {"orderId": payload.order_id},
                      UpdateModifications::Document(to_document(&payload)?),
                      UpdateOptions::builder().upsert(true).build(),
                    )
                    .await;
                  Ok(())
                }
                Err(e) => Err(Box::new(e)),
              };
              return resp;
            })
            .await;
          }
          .boxed()
        })
        .collect();
    let res_err = join_all(req_lst)
      .await
      .into_iter()
      .find(|item| item.is_err());
    return match res_err {
      Some(e) => Err(e.unwrap_err()),
      None => Ok(pos_gid),
    };
  }

  async fn remove_order(
    &mut self,
    api_key_id: ObjectId,
    gid: ObjectId,
  ) -> ThreadSafeResult<ExecutionResult> {
    let api_key = self
      .keychain
      .get(Exchanges::Binance, api_key_id)
      .await?
      .ok_or(ObjectNotFound::new("API key".to_string()))?;
    let api_key = api_key.inner();
    let mut positions = self
      .positions
      .find(doc! {"positionGroupId": gid}, None)
      .await?
      .filter_map(|pos| async { pos.ok() })
      .boxed();
    let cli = match self.get_client(api_key.pub_key.clone()) {
      Err(e) => return Err(e),
      Ok(o) => o,
    };
    let mut order_cancel_vec = vec![];
    let mut position_reverse_vec = vec![];
    while let Some(pos) = positions.next().await {
      // Cancel Order
      let me = self.clone();
      let symbol = pos.symbol.clone();
      let order_id = pos.order_id.clone();
      let cancel_cli = cli.clone();
      let reverse_cli = cli.clone();
      order_cancel_vec.push(retry_async(5, move || {
        let symbol = symbol.clone();
        let order_id = order_id.clone();
        let me = me.clone();
        let cancel_cli = cancel_cli.clone();
        async move {
          let req =
            CancelOrderRequest::<i64>::new(symbol).order_id(Some(order_id));
          let qs = to_qs(&req)?;
          let qs = format!(
            "{}&signature={}",
            qs.clone(),
            me.sign(qs, api_key.prv_key.clone())
          );
          let resp = cancel_cli
            .delete(format!("{}/api/v3/order?{}", REST_ENDPOINT, qs))
            .send()
            .await?;
          let status = resp.status();
          if !status.is_success() {
            return Err(Box::new(StatusFailure {
              url: Some(resp.url().clone()),
              code: status.as_u16(),
              text: resp
                .text()
                .await
                .unwrap_or("Failed to get the text".to_string()),
            })
              as Box<dyn ::std::error::Error + Send + Sync>);
          }
          return Ok(resp);
        }
      }));
      let symbol = pos.symbol.clone();
      if let Some(fills) = &pos.fills {
        // Sell the position
        let qty_to_reverse =
          fills.into_iter().map(|item| item.qty).sum::<f64>();
        let req = OrderRequest::<i64>::new(
          symbol.clone(),
          Side::Sell,
          OrderType::Market,
        )
        .quantity(Some(qty_to_reverse));
        let qs = to_qs(&req)?;
        let qs = format!(
          "{}&signature={}",
          qs.clone(),
          self.sign(qs, api_key.prv_key.clone())
        );
        let pos: Order = pos.clone().into();
        let pos_pur_price: OrderInner = pos.clone().sum();
        position_reverse_vec.push(retry_async(5, move || {
          let qs = qs.clone();
          let pos_pur_price = pos_pur_price.clone();
          let reverse_cli = reverse_cli.clone();
          async move {
            let resp = reverse_cli
              .post(format!("{}/api/v3/order?{}", REST_ENDPOINT, &qs))
              .send()
              .await?;
            let status = resp.status();
            if !status.is_success() {
              return Err(Box::new(StatusFailure {
                url: Some(resp.url().clone()),
                code: status.as_u16(),
                text: resp
                  .text()
                  .await
                  .unwrap_or("Failed to get the text".to_string()),
              })
                as Box<dyn ::std::error::Error + Send + Sync>);
            }
            let rev_order_resp: OrderResponse<String, i64> =
              resp.json().await?;
            let rev_order_resp =
              OrderResponse::<f64, DateTime>::try_from(rev_order_resp)?;
            let rev_order_resp: Order = rev_order_resp.into();
            let rev_pos_price: OrderInner = rev_order_resp.sum();
            return Ok((pos_pur_price, rev_pos_price));
          }
        }));
      };
    }
    let (order_res, position_res) =
      join(join_all(order_cancel_vec), join_all(position_reverse_vec)).await;
    for order_res in order_res {
      if order_res.is_err() {
        return Err(order_res.err().unwrap());
      }
    }
    let mut pur_order = OrderInner::default();
    let mut sell_order = OrderInner::default();
    for position_res in position_res {
      let (pur, sell) = match position_res {
        Err(e) => return Err(e),
        Ok(o) => o,
      };
      pur_order += pur;
      sell_order += sell;
    }
    return Ok(ExecutionResult::calculate_profit(&sell_order, &pur_order));
  }
}

impl Sign for Executor {
  fn get_secret_key(&self, prv_key: String) -> hmac::Key {
    return hmac::Key::new(hmac::HMAC_SHA256, prv_key.as_bytes());
  }
}
