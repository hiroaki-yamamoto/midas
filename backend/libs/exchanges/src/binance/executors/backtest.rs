use ::std::collections::HashMap;
use ::std::pin::Pin;

use ::async_stream::stream;
use ::async_trait::async_trait;
use ::futures::stream::{Stream, StreamExt};
use ::mongodb::bson::oid::ObjectId;

use ::types::GenericResult;

use crate::binance::entities::Kline;
use crate::binance::history_recorder::HistoryRecorder;
use crate::entities::{ExecutionResult, OrderOption};
use crate::traits::Executor as ExecutorTrait;

#[derive(Debug, Clone)]
struct Order {
  symbol: String,
  price: f64,
  qty: f64,
}

pub struct Executor {
  spread: f64,
  maker_fee: f64,
  taker_fee: f64,
  cur_trade: Option<Kline>,
  orders: HashMap<ObjectId, Vec<Order>>,
  positions: HashMap<ObjectId, Order>,
  hist_recorder: HistoryRecorder,
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
    });
  }

  pub async fn open(
    &mut self,
  ) -> GenericResult<impl Stream<Item = Kline> + '_> {
    let mut stream = self.hist_recorder.list(None).await?.boxed();
    self.cur_trade = None;
    return Ok(stream! {
      while let Some(v) = stream.next().await {
        self.cur_trade = Some(v.clone());
        yield v;
      }
      self.cur_trade = None;
    });
  }
}

#[async_trait]
impl ExecutorTrait for Executor {
  async fn create_order(
    &self,
    symbol: String,
    price: Option<f64>,
    budget: f64,
    order_option: Option<OrderOption>,
  ) -> GenericResult<ObjectId> {
    let id = ObjectId::new();
    return Ok(id);
  }
  async fn remove_order(&self, id: ObjectId) -> GenericResult<ExecutionResult> {
    return Ok(ExecutionResult::default());
  }
}
