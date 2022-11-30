use ::std::collections::HashMap;

use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt};
use ::mongodb::bson::oid::ObjectId;
use ::nats::jetstream::JetStream as NatsJS;

use ::entities::{
  BookTicker, ExecutionSummary, ExecutionType, Order, OrderInner, OrderOption,
};
use ::errors::ExecutionFailed;
use ::observers::traits::TradeObserver as TradeObserverTrait;

use crate::errors::ExecutionResult;
use crate::traits::{
  Executor as ExecutorTrait, TestExecutor as TestExecutorTrait,
};

use ::observers::binance::TradeObserver;

pub struct Executor {
  observer: TradeObserver,
  orders: HashMap<ObjectId, Order>,
  positions: HashMap<ObjectId, OrderInner>,
  cur_trade: Option<BookTicker>,
  maker_fee: f64,
  taker_fee: f64,
}

impl Executor {
  pub async fn new(broker: &NatsJS, maker_fee: f64, taker_fee: f64) -> Self {
    return Self {
      observer: TradeObserver::new(None, broker).await,
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
    return self.cur_trade.clone();
  }

  fn maker_fee(&self) -> f64 {
    return self.maker_fee;
  }
  fn taker_fee(&self) -> f64 {
    return self.taker_fee;
  }
  fn get_orders(&self) -> HashMap<ObjectId, Order> {
    return self.orders.clone();
  }
  fn get_positions(&self) -> HashMap<ObjectId, OrderInner> {
    return self.positions.clone();
  }
  fn set_orders(&mut self, orders: HashMap<ObjectId, Order>) {
    self.orders = orders;
  }
  fn set_positions(&mut self, positions: HashMap<ObjectId, OrderInner>) {
    self.positions = positions;
  }
}

#[async_trait]
impl ExecutorTrait for Executor {
  async fn open(
    &mut self,
  ) -> ExecutionResult<BoxStream<ExecutionResult<BookTicker>>> {
    let observer = self.observer.clone();
    let stream = try_stream! {
      let mut src_stream = observer.subscribe().await?;
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
    _: ObjectId,
    _: String,
    _: Option<f64>,
    _: f64,
    _: Option<OrderOption>,
  ) -> ExecutionResult<ObjectId> {
    return Err(
      ExecutionFailed::new("Call create_order from TestExecutorTrait.").into(),
    );
  }

  async fn remove_order(
    &mut self,
    _: ObjectId,
    _: ObjectId,
  ) -> ExecutionResult<ExecutionSummary> {
    return Err(
      ExecutionFailed::new("Call remove_position from TestExecutorTrait.")
        .into(),
    );
  }
}
