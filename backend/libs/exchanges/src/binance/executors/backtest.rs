use ::std::collections::HashMap;

use ::async_stream::try_stream;
use ::async_trait::async_trait;
use ::futures::stream::{LocalBoxStream, StreamExt};
use ::mongodb::bson::oid::ObjectId;

use ::rpc::entities::{BackTestPriceBase, Exchanges};
use ::types::GenericResult;

use crate::binance::history_recorder::HistoryRecorder;
use crate::entities::{BookTicker, ExecutionResult, OrderOption};
use crate::errors::ExecutionFailed;
use crate::traits::Executor as ExecutorTrait;

use super::entities::{Order, OrderInner};

#[derive(Clone, Debug)]
enum ExecutionType {
  Maker,
  Taker,
}

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

  fn execute_order(&mut self, exe_type: ExecutionType) -> GenericResult<()> {
    if self.cur_trade.is_none() {
      return Err(Box::new(ExecutionFailed::new(
        "Trade Stream seems to be closed.",
      )));
    }
    let cur_trade = self.cur_trade.clone().unwrap();
    for (key, order) in self.orders.iter_mut() {
      if order.symbol != cur_trade.symbol {
        continue;
      }
      let fee = match exe_type {
        ExecutionType::Maker => self.maker_fee,
        ExecutionType::Taker => self.taker_fee,
      };
      let position = order
        .inner
        .iter()
        .filter(|&order| order.price >= cur_trade.ask_price)
        .fold(OrderInner::default(), |mut acc, order| {
          let mut order = order.clone();
          order.qty = order.qty * (1.0 - fee);
          acc += order;
          return acc;
        });
      let remain: Vec<OrderInner> = order
        .inner
        .iter()
        .filter(|&order| order.price < cur_trade.ask_price)
        .cloned()
        .collect();
      match self.positions.get_mut(key) {
        None => {
          self.positions.insert(key.clone(), position);
        }
        Some(v) => {
          *v += position;
        }
      }
      order.inner = remain;
    }
    return Ok(());
  }
}

#[async_trait]
impl ExecutorTrait for Executor {
  async fn open(
    &mut self,
  ) -> GenericResult<LocalBoxStream<'_, GenericResult<BookTicker>>> {
    let half_spread = self.spread / 2.0;
    let price_base = self.price_base_policy.clone();
    let mut stream = self
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
      while let Some(v) = stream.next().await {
        self.cur_trade = Some(v.clone());
        self.execute_order(ExecutionType::Maker)?;
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
    if self.cur_trade.is_none() {
      return Err(Box::new(ExecutionFailed::new("Trade Stream is closed.")));
    }
    let id = ObjectId::new();
    let price = price.unwrap_or(self.cur_trade.as_ref().unwrap().ask_price);
    let orders = match order_option {
      None => vec![OrderInner {
        price,
        qty: budget / price,
      }],
      Some(v) => {
        let price_diff = price * v.price_ratio;
        v.calc_trading_amounts(budget)
          .into_iter()
          .enumerate()
          .map(|(index, amount)| {
            let order_price = (price - price_diff) * ((index + 1) as f64);
            OrderInner {
              price: order_price.clone(),
              qty: amount / order_price,
            }
          })
          .collect()
      }
    };
    let orders = Order::new(symbol, orders);
    self.orders.insert(id.clone(), orders);
    self.execute_order(ExecutionType::Taker)?;
    return Ok(id);
  }
  async fn remove_order(
    &mut self,
    id: ObjectId,
  ) -> GenericResult<ExecutionResult> {
    if self.cur_trade.is_none() {
      return Err(Box::new(ExecutionFailed::new("Trade stream is closed.")));
    }
    let cur_trade = self.cur_trade.clone().unwrap();
    let price = cur_trade.bid_price;
    self.orders.remove(&id);
    let fee = self.taker_fee;
    let ret = match self.positions.get_mut(&id) {
      None => ExecutionResult::default(),
      Some(v) => {
        let qty = v.qty * (1.0 - fee);
        let profit = ((qty * price) - (v.qty * v.price)) / qty;
        let profit_ratio = profit / v.price;
        ExecutionResult {
          id,
          price,
          profit,
          profit_ratio,
          qty: v.qty,
        }
      }
    };
    return Ok(ret);
  }
}
