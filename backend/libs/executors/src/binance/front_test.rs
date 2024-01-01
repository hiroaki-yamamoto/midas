use ::std::collections::HashMap;

use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt};
use ::mongodb::bson::oid::ObjectId;
use ::rug::Float;

use ::entities::{
  BookTicker, ExecutionSummary, ExecutionType, Order, OrderInner, OrderOption,
};
use ::errors::{ExecutionFailed, ExecutionResult};
use ::observers::traits::ITradeSubscriber as TradeSubscriberTrait;
use ::subscribe::nats::Client as Nats;

use crate::traits::{
  Executor as ExecutorTrait, TestExecutor as TestExecutorTrait,
};

use ::observers::binance::TradeSubscriber;

pub struct Executor {
  observer: TradeSubscriber,
  orders: HashMap<ObjectId, Order>,
  positions: HashMap<ObjectId, OrderInner>,
  cur_trade: Option<BookTicker>,
  maker_fee: Float,
  taker_fee: Float,
}

impl Executor {
  pub async fn new(
    broker: Nats,
    maker_fee: Float,
    taker_fee: Float,
  ) -> ExecutionResult<Self> {
    return Ok(Self {
      observer: TradeSubscriber::new(&broker).await?,
      orders: HashMap::new(),
      positions: HashMap::new(),
      cur_trade: None,
      maker_fee,
      taker_fee,
    });
  }
}

impl TestExecutorTrait for Executor {
  fn get_current_trade(&self) -> Option<BookTicker> {
    return self.cur_trade.clone();
  }

  fn maker_fee(&self) -> Float {
    return self.maker_fee.clone();
  }
  fn taker_fee(&self) -> Float {
    return self.taker_fee.clone();
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
    _: ObjectId,
    _: String,
    _: Option<Float>,
    _: Float,
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
    _: ObjectId,
  ) -> ExecutionResult<ExecutionSummary> {
    return Err(
      ExecutionFailed::new("Call remove_position from TestExecutorTrait.")
        .into(),
    );
  }
}
