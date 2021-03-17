use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::stream::LocalBoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::{oid::ObjectId, DateTime};
use ::mongodb::{Collection, Database};
use ::nats::asynk::Connection as NatsCon;
use ::ring::hmac;
use ::serde_qs::to_string as to_qs;
use ::slog::Logger;

use ::entities::{BookTicker, OrderOption};
use ::errors::ObjectNotFound;
use ::executor::Executor as ExecutorTrait;
use ::keychain::KeyChain;
use ::rpc::entities::Exchanges;
use ::sign::Sign;
use ::types::GenericResult;

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
  orders: Collection,
  positions: Collection,
}

impl Executor {
  pub async fn new(broker: NatsCon, db: Database, log: Logger) -> Self {
    let keychain = KeyChain::new(broker.clone(), db.clone()).await;
    let orders = db.collection("binance.orders");
    let positions = db.collection("binance.positions");
    return Self {
      broker,
      keychain,
      db,
      log,
      orders,
      positions,
    };
  }
}

impl PubClient for Executor {}

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
  ) -> GenericResult<ObjectId> {
    let key = self.keychain.get(Exchanges::Binance, api_key_id).await?;
    let key = key
      .ok_or(ObjectNotFound::new("API KeyPair".to_string()))?
      .inner();
    let order_type = order_option
      .map(|_| OrderType::Limit)
      .unwrap_or(OrderType::Market);
    let req_lst = order_option
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
            order = *order.order_response_type(Some(OrderResponseType::RESULT));
            return order;
          })
          .collect()
      })
      .unwrap_or_else(|| {
        vec![*OrderRequest::<i64>::new(symbol, Side::Buy, order_type)
          .order_response_type(Some(OrderResponseType::RESULT))]
      })
      .into_iter()
      .map(|order| async {
        let qs = to_qs(&order)?;
        let signature = self.sign(qs, key.prv_key);
        let qs = format!("{}&signature={}", qs, signature);
        let cli = self.get_client(key.pub_key)?;
        for _ in 0..5 {
          let resp = cli
            .post(format!("{}/api/v3/order?{}", REST_ENDPOINT, qs))
            .send()
            .await;
          if let Ok(resp) = resp {
            let payload: OrderResponse<String, i64> = resp.json().await?;
            let payload: OrderResponse<f64, DateTime> = payload.into()?;
          }
        }
        return Ok(());
      });
  }
}

impl Sign for Executor {
  fn get_secret_key(&self, prv_key: String) -> hmac::Key {
    return hmac::Key::new(hmac::HMAC_SHA256, prv_key.as_bytes());
  }
}
