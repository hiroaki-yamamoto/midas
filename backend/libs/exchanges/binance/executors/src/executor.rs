use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::stream::LocalBoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::{Collection, Database};
use ::nats::asynk::Connection as NatsCon;
use ::ring::hmac;
use ::slog::Logger;

use ::entities::{BookTicker, OrderOption};
use ::errors::ObjectNotFound;
use ::executor::Executor as ExecutorTrait;
use ::keychain::KeyChain;
use ::rpc::entities::Exchanges;
use ::sign::Sign;
use ::types::GenericResult;

use ::binance_observers::{TradeObserver, TradeObserverTrait};

use super::entities::{OrderRequest, OrderType, Side};

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
    let order_type = order_option
      .map(|_| OrderType::Limit)
      .unwrap_or(OrderType::Market);
    let orders: Vec<OrderRequest<i64>> = match order_option {
      None => vec![OrderRequest::<i64>::new(symbol, Side::Buy, order_type)],
      Some(o) => o
        .calc_trading_amounts(budget)
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
          return order;
        })
        .collect(),
    };
  }
}

impl Sign for Executor {
  fn get_secret_key(&self, prv_key: String) -> hmac::Key {
    return hmac::Key::new(hmac::HMAC_SHA256, prv_key.as_bytes());
  }
}
