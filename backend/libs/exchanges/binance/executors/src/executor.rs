use ::std::error::Error;
use std::convert::TryFrom;

use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::future::{join_all, BoxFuture};
use ::futures::stream::LocalBoxStream;
use ::futures::{Future, FutureExt, StreamExt};
use ::mongodb::bson::{doc, oid::ObjectId, to_document, DateTime};
use ::mongodb::options::{UpdateModifications, UpdateOptions};
use ::mongodb::{Collection, Database};
use ::nats::asynk::Connection as NatsCon;
use ::ring::hmac;
use ::serde_qs::to_string as to_qs;
use ::slog::Logger;

use ::base_recorder::Recorder as RecorderTrait;
use ::entities::{BookTicker, OrderOption};
use ::errors::ObjectNotFound;
use ::executor::Executor as ExecutorTrait;
use ::keychain::KeyChain;
use ::rpc::entities::Exchanges;
use ::sign::Sign;
use ::types::{GenericResult, ThreadSafeResult};

use ::binance_clients::{constants::REST_ENDPOINT, PubClient};
use ::binance_observers::{TradeObserver, TradeObserverTrait};

use super::entities::{
  OrderRequest, OrderResponse, OrderResponseType, OrderType, Side,
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
    me.update_indices(&["orderId", "clientOrderId"]).await;
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
      .map(|_| OrderType::Limit)
      .unwrap_or(OrderType::Market);
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
              order =
                *order.order_response_type(Some(OrderResponseType::RESULT));
              return order;
            })
            .collect::<Vec<OrderRequest<i64>>>()
        })
        .unwrap_or_else(|| {
          vec![*OrderRequest::<i64>::new(symbol, Side::Buy, order_type)
            .order_response_type(Some(OrderResponseType::RESULT))]
        })
        .into_iter()
        .map(|order| {
          async {
            let qs = to_qs(&order)?;
            let signature = self.sign(qs, key.prv_key);
            let qs = format!("{}&signature={}", qs, signature);
            let cli = self.get_client(key.pub_key)?;
            let mut err: Box<dyn ::std::error::Error + Send + Sync>;
            for _ in 0usize..5 {
              let resp = cli
                .post(format!("{}/api/v3/order?{}", REST_ENDPOINT, qs))
                .send()
                .await;
              match resp {
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
                  return Ok(());
                }
                Err(e) => {
                  err = Box::new(e);
                  continue;
                }
              }
            }
            return Err(err);
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
}

impl Sign for Executor {
  fn get_secret_key(&self, prv_key: String) -> hmac::Key {
    return hmac::Key::new(hmac::HMAC_SHA256, prv_key.as_bytes());
  }
}
