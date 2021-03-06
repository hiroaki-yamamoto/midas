use ::std::collections::HashMap;

use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::stream::{LocalBoxStream, StreamExt};
use ::mongodb::bson::oid::ObjectId;

use ::rpc::entities::{BackTestPriceBase, Exchanges};
use ::types::GenericResult;

use ::binance_histories::recorder::HistoryRecorder;
use ::entities::{
  BookTicker, ExecutionResult, ExecutionType, Order, OrderInner, OrderOption,
};
use ::errors::ExecutionFailed;
use ::executor::{
  Executor as ExecutorTrait, TestExecutor as TestExecutorTrait,
};

pub struct Executor {
  spread: f64,
  maker_fee: f64,
  taker_fee: f64,
  cur_trade: Option<BookTicker>,
  orders: HashMap<ObjectId, Order>,
  positions: HashMap<ObjectId, OrderInner>,
  hist_recorder: HistoryRecorder,
  pub price_base_policy: BackTestPriceBase,
}

impl Executor {
  pub async fn new(
    history_recorder: HistoryRecorder,
    spread: f64,
    maker_fee: f64,
    taker_fee: f64,
  ) -> GenericResult<Self> {
    return Ok(Self {
      spread,
      maker_fee,
      taker_fee,
      cur_trade: None,
      orders: HashMap::new(),
      positions: HashMap::new(),
      hist_recorder: history_recorder,
      price_base_policy: BackTestPriceBase::HighLowMid,
    });
  }
}

#[async_trait]
impl ExecutorTrait for Executor {
  async fn open(
    &mut self,
  ) -> GenericResult<LocalBoxStream<'_, GenericResult<BookTicker>>> {
    let half_spread = self.spread / 2.0;
    let price_base = self.price_base_policy.clone();
    let mut db_stream = self
      .hist_recorder
      .list(None)
      .await?
      .map(move |kline| {
        let kline = &kline;
        let price = match price_base {
          BackTestPriceBase::Close => kline.close_price,
          BackTestPriceBase::Open => kline.open_price,
          BackTestPriceBase::High => kline.high_price,
          BackTestPriceBase::Low => kline.low_price,
          BackTestPriceBase::OpenCloseMid => {
            (kline.close_price + kline.open_price) / 2.0
          }
          BackTestPriceBase::HighLowMid => {
            (kline.high_price + kline.low_price) / 2.0
          }
        };
        return BookTicker {
          exchange: Exchanges::Binance,
          symbol: kline.symbol.clone(),
          id: ObjectId::new().to_string(),
          bid_price: price - half_spread,
          ask_price: price + half_spread,
          ask_qty: kline.volume,
          bid_qty: kline.volume,
        };
      })
      .boxed();
    self.cur_trade = None;
    let stream = try_stream! {
      while let Some(v) = db_stream.next().await {
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
  ) -> GenericResult<ObjectId> {
    return Err(Box::new(ExecutionFailed::new(
      "Call create_order from TestExecutorTrait.",
    )));
  }

  async fn remove_order(
    &mut self,
    _: ObjectId,
    _: ObjectId,
  ) -> GenericResult<ExecutionResult> {
    return Err(Box::new(ExecutionFailed::new(
      "Call remove_position from TestExecutorTrait.",
    )));
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
