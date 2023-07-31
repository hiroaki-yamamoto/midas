use ::std::collections::HashMap;
use std::convert::TryFrom;

use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt};
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::Database;
use ::rug::Float;

use ::rpc::entities::{BackTestPriceBase, Exchanges};

use ::entities::{
  BookTicker, ExecutionSummary, ExecutionType, Order, OrderInner, OrderOption,
};
use ::errors::{ExecutionFailed, ExecutionResult};
use ::history::binance::entities::Kline;
use ::history::binance::writer::HistoryWriter;
use ::history::traits::HistoryWriter as HistoryWriterTrait;

use crate::traits::{
  Executor as ExecutorTrait, TestExecutor as TestExecutorTrait,
};

pub struct Executor {
  spread: Float,
  maker_fee: Float,
  taker_fee: Float,
  cur_trade: Option<BookTicker>,
  orders: HashMap<ObjectId, Order>,
  positions: HashMap<ObjectId, OrderInner>,
  writer: HistoryWriter,
  pub price_base_policy: BackTestPriceBase,
}

impl Executor {
  pub async fn new(
    db: &Database,
    spread: Float,
    maker_fee: Float,
    taker_fee: Float,
    price_base_policy: BackTestPriceBase,
  ) -> Self {
    return Self {
      spread,
      maker_fee,
      taker_fee,
      cur_trade: None,
      orders: HashMap::new(),
      positions: HashMap::new(),
      writer: HistoryWriter::new(db).await,
      price_base_policy,
    };
  }
}

#[async_trait]
impl ExecutorTrait for Executor {
  async fn open(
    &mut self,
  ) -> ExecutionResult<BoxStream<ExecutionResult<BookTicker>>> {
    let half_spread = self.spread.clone() / 2.0;
    let price_base = self.price_base_policy.clone();
    let writer = self.writer.clone();
    let db_stream = writer
      .list(None)
      .await?
      .filter_map(|klines| async { Vec::<Kline>::try_from(klines).ok() })
      .map(move |klines| {
        let klines = klines.into_iter().map(|kline| {
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
          let ticker = BookTicker {
            exchange: Exchanges::Binance,
            symbol: kline.symbol.clone(),
            id: ObjectId::new().to_string(),
            bid_price: price.clone() - &half_spread,
            ask_price: price.clone() + &half_spread,
            ask_qty: kline.volume.clone(),
            bid_qty: kline.volume.clone(),
          };
          return ticker;
        });
        return ::futures::stream::iter(klines.collect::<Vec<BookTicker>>());
      })
      .flatten()
      .map(move |ticker| {
        self.cur_trade = Some(ticker.clone());
        self.execute_order(ExecutionType::Taker)?;
        return Ok(ticker);
      })
      .boxed();
    return Ok(db_stream);
  }

  async fn create_order(
    &mut self,
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
  ) -> ExecutionResult<ExecutionSummary> {
    return Err(
      ExecutionFailed::new("Call remove_position from TestExecutorTrait.")
        .into(),
    );
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
