use ::std::collections::HashMap;

use ::async_stream::stream;
use ::async_trait::async_trait;
use ::futures::stream::{LocalBoxStream, StreamExt};
use ::mongodb::bson::oid::ObjectId;
use ::nats::asynk::Connection as NatsCon;
use ::slog::Logger;

use ::types::{ret_on_err, GenericResult};

use crate::traits::{
  Executor as ExecutorTrait, TradeObserver as TradeObserverTrait,
};

use crate::entities::BookTicker;

use super::super::TradeObserver;
use super::entities::{Order, OrderInner};

pub struct Executor {
  observer: TradeObserver,
  orders: HashMap<ObjectId, Order>,
  positions: HashMap<ObjectId, OrderInner>,
  cur_trade: Option<BookTicker>,
  maker_fee: f64,
  taker_fee: f64,
}

impl Executor {
  pub async fn new(
    logger: Logger,
    broker: NatsCon,
    maker_fee: f64,
    taker_fee: f64,
  ) -> Self {
    return Self {
      observer: TradeObserver::new(None, broker, logger).await,
      orders: HashMap::new(),
      positions: HashMap::new(),
      cur_trade: None,
      maker_fee,
      taker_fee,
    };
  }
}

#[async_trait]
impl ExecutorTrait for Executor {
  async fn open(
    &mut self,
  ) -> GenericResult<LocalBoxStream<'_, GenericResult<BookTicker>>> {
    let observer = self.observer.clone();
    let mut src_stream =
      ret_on_err!(observer.subscribe().await).map(|ticker| return ticker);
    let stream = stream! {
      while let Some(v) = src_stream.next().await {
        self.cur_trade = Some(v.clone());
        yield GenericResult::<BookTicker>::Ok(v);
      }
      self.cur_trade = None;
    };
    return Ok(Box::pin(stream.boxed_local());
  }

  async fn create_order(
    &mut self,
    symbol: String,
    price: Option<f64>,
    budget: f64,
    order_option: Option<OrderOption>,
  ) -> GenericResult<ObjectId> {
  }

  async fn remove_order(
    &mut self,
    id: ObjectId,
  ) -> GenericResult<ExecutionResult> {
  }
}
