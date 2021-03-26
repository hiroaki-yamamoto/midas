use ::std::error::Error;
use std::convert::TryFrom;

use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::future::{join, join_all, BoxFuture};
use ::futures::stream::LocalBoxStream;
use ::futures::{FutureExt, StreamExt};
use ::mongodb::bson::{
  doc, from_document, oid::ObjectId, to_document, DateTime, Document,
};
use ::mongodb::options::{UpdateModifications, UpdateOptions};
use ::mongodb::{Collection, Database};
use ::nats::asynk::Connection as NatsCon;
use ::ring::hmac;
use ::serde_qs::to_string as to_qs;
use ::slog::Logger;

use ::base_recorder::Recorder as RecorderTrait;
use ::entities::{BookTicker, ExecutionResult, Order, OrderInner, OrderOption};
use ::errors::{ObjectNotFound, StatusFailure};
use ::executor::Executor as ExecutorTrait;
use ::keychain::KeyChain;
use ::rpc::entities::Exchanges;
use ::sign::Sign;
use ::types::retry::retry_async;
use ::types::{GenericResult, ThreadSafeResult};

use ::binance_clients::{constants::REST_ENDPOINT, PubClient};
use ::binance_observers::{TradeObserver, TradeObserverTrait};

use super::entities::{
  CancelOrderRequest, OrderRequest, OrderResponse, OrderResponseType,
  OrderType, Side,
};

pub struct Executor {
  keychain: KeyChain,
  broker: NatsCon,
  db: Database,
  log: Logger,
  positions: Collection,
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
impl RecorderTrait for Executor {
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
  ) -> GenericResult<LocalBoxStream<'_, GenericResult<BookTicker>>> {
    let observer = TradeObserver::new(
      Some(self.db.clone()),
      self.broker.clone(),
      self.log.clone(),
    )
    .await;
    let sub = observer.subscribe().await?;
    let stream = try_stream! {
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
    let key = key
      .ok_or(ObjectNotFound::new("API KeyPair".to_string()))?
      .inner();
    let order_type = order_option
      .map(|_| price.map(|_| OrderType::Limit).unwrap_or(OrderType::Market))
      .unwrap_or(OrderType::Market);
    let cli = self.get_client(key.pub_key)?;
    let req_lst: Vec<BoxFuture<Result<(), Box<dyn Error + Send + Sync>>>> =
      order_option
        .map(|o| {
          o.calc_trading_amounts(budget)
            .into_iter()
            .enumerate()
            .map(|(index, tr_amount)| {
              let mut order =
                OrderRequest::<i64>::new(symbol, Side::Buy, order_type);
              if o.iceberg {
                order.iceberg_qty(Some(tr_amount));
              } else {
                order.quantity(Some(tr_amount));
              }
              if order_type == OrderType::Limit {
                order.price(Some(o.calc_order_price(price.unwrap(), index)));
              }
              order =
                *order.order_response_type(Some(OrderResponseType::RESULT));
              return order;
            })
            .collect::<Vec<OrderRequest<i64>>>()
        })
        .unwrap_or_else(|| {
          let order = OrderRequest::<i64>::new(symbol, Side::Buy, order_type)
            .order_response_type(Some(OrderResponseType::RESULT));
          if order_type == OrderType::Limit {
            order.price(price);
          }
          return vec![*order];
        })
        .into_iter()
        .map(|order| {
          async {
            let qs = to_qs(&order)?;
            let signature = self.sign(qs, key.prv_key);
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
                  payload.position_group_id = Some(pos_gid);
                  self
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
  ) -> GenericResult<ExecutionResult> {
    let api_key = self
      .keychain
      .get(Exchanges::Binance, api_key_id)
      .await?
      .ok_or(ObjectNotFound::new("API key".to_string()))?
      .inner();
    let positions = self
      .positions
      .find(doc! {"positionGroupId": gid}, None)
      .await?
      .filter_map(|pos| async { pos.ok() })
      .filter_map(|pos| async {
        from_document::<OrderResponse<f64, DateTime>>(pos).ok()
      })
      .boxed_local();
    let cli = self.get_client(api_key.pub_key)?;
    let order_cancel_vec = vec![];
    let position_reverse_vec = vec![];
    while let Some(pos) = positions.next().await {
      // Cancel Order
      let cancel_order = async {
        let req = CancelOrderRequest::<i64>::new(pos.symbol)
          .order_id(Some(pos.order_id));
        let qs = to_qs(&req)?;
        let qs = format!("{}&signature={}", qs, self.sign(qs, api_key.prv_key));
        order_cancel_vec.push(retry_async(5, || async {
          let resp = cli
            .delete(format!("{}/api/v3/order?{}", REST_ENDPOINT, qs))
            .send()
            .await?;
          let status = resp.status();
          if !status.is_success() {
            return Err(Box::new(StatusFailure {
              url: *resp.url(),
              code: status.as_u16(),
              text: resp
                .text()
                .await
                .unwrap_or("Failed to get the text".to_string()),
            }) as Box<dyn ::std::error::Error>);
          }
          return Ok(resp);
        }));
        Ok(())
      };
      if let Some(fills) = pos.fills {
        // Sell the position
        let qty_to_reverse =
          fills.into_iter().map(|item| item.qty).sum::<f64>();
        let req =
          OrderRequest::<i64>::new(pos.symbol, Side::Sell, OrderType::Market)
            .quantity(Some(qty_to_reverse));
        let qs = to_qs(&req)?;
        position_reverse_vec.push(retry_async(5, || async {
          let resp = cli
            .post(format!("{}/api/v3/order?{}", REST_ENDPOINT, qs))
            .send()
            .await?;
          let status = resp.status();
          if !status.is_success() {
            return Err(Box::new(StatusFailure {
              url: *resp.url(),
              code: status.as_u16(),
              text: resp
                .text()
                .await
                .unwrap_or("Failed to get the text".to_string()),
            }) as Box<dyn ::std::error::Error>);
          }
          let rev_order_resp: OrderResponse<String, i64> = resp.json().await?;
          let rev_order_resp =
            OrderResponse::<f64, DateTime>::try_from(rev_order_resp)?;
          let rev_pos_price: OrderInner = rev_order_resp.into().sum();
          let pos_pur_price: OrderInner = pos.into().sum();
          return Ok((pos_pur_price, rev_pos_price));
        }));
      };
    }
    let (order_res, position_res) =
      join(join_all(order_cancel_vec), join_all(position_reverse_vec)).await;
    for order_res in order_res {
      order_res?;
    }
    let mut pur_order = OrderInner::default();
    let mut sell_order = OrderInner::default();
    for position_res in position_res {
      let (pur, sell) = position_res?;
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
