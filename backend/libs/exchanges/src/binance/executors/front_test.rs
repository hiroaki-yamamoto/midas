use ::std::collections::HashMap;

use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::stream::{LocalBoxStream, StreamExt};
use ::mongodb::bson::oid::ObjectId;
use ::nats::asynk::Connection as NatsCon;
use ::slog::Logger;

use ::types::{ret_on_err, GenericResult};

use crate::traits::{
  Executor as ExecutorTrait, TestExecutor as TestExecutorTrait,
  TradeObserver as TradeObserverTrait,
};

use crate::entities::{
  BookTicker, ExecutionResult, ExecutionType, Order, OrderInner, OrderOption,
};

use super::super::TradeObserver;

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

impl TestExecutorTrait for Executor {
  fn get_current_trade(&self) -> Option<BookTicker> {
    return self.cur_trade;
  }

  fn maker_fee(&self) -> f64 {
    return self.maker_fee;
  }
  fn taker_fee(&self) -> f64 {
    return self.taker_fee;
  }
  fn get_orders(&mut self) -> &mut HashMap<ObjectId, Order> {
    return &mut self.orders;
  }
  fn get_positions(&mut self) -> &mut HashMap<ObjectId, OrderInner> {
    return &mut self.positions;
  }
}

#[async_trait]
impl ExecutorTrait for Executor {
  async fn open(
    &mut self,
  ) -> GenericResult<LocalBoxStream<'_, GenericResult<BookTicker>>> {
    let observer = self.observer.clone();
    let stream = try_stream! {
      let mut src_stream =
        observer.subscribe().await?.map(|ticker| return ticker);
      while let Some(v) = src_stream.next().await {
        self.cur_trade = Some(v.clone());
        self.execute_order(ExecutionType::Taker)?;
        yield v;
      }
      self.cur_trade = None;
    };
    return Ok(Box::pin(stream));
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
