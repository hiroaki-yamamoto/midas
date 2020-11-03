use ::std::collections::HashMap;
use ::std::convert::AsRef;

use ::async_stream::stream;
use ::async_trait::async_trait;
use ::futures::stream::{Stream, StreamExt};
use ::mongodb::bson::oid::ObjectId;

use ::rpc::entities::BackTestPriceBase;
use ::types::GenericResult;

use crate::binance::history_recorder::HistoryRecorder;
use crate::entities::{ExecutionResult, OrderOption};
use crate::errors::ExecutionFailed;
use crate::traits::Executor as ExecutorTrait;

#[derive(Debug, Clone, Default)]
struct Order {
  symbol: String,
  price: f64,
  qty: f64,
}

impl AsRef<Self> for Order {
  fn as_ref(&self) -> &Self {
    return self;
  }
}

impl<T> ::std::ops::Add<T> for Order
where
  T: AsRef<Self>,
{
  type Output = Self;
  fn add(self, rhs: T) -> Self::Output {
    let rhs = rhs.as_ref();
    return Self {
      symbol: self.symbol,
      qty: self.qty + rhs.qty,
      price: ((self.qty * self.price) + (rhs.qty * rhs.price))
        / (self.qty + rhs.qty),
    };
  }
}

impl<T> ::std::ops::AddAssign<T> for Order
where
  T: AsRef<Self>,
{
  fn add_assign(&mut self, rhs: T) {
    let rhs = rhs.as_ref();
    self.price = (self.qty * self.price) + (rhs.qty * rhs.price);
    self.price /= self.qty + rhs.qty;
    self.qty += rhs.qty;
  }
}

#[derive(Debug, Clone)]
pub struct Price {
  symbol: String,
  ask: f64,
  bid: f64,
  price_base: BackTestPriceBase,
  asset_volume: f64,
  base_volume: f64,
}

pub struct Executor {
  spread: f64,
  maker_fee: f64,
  taker_fee: f64,
  cur_trade: Option<Price>,
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
    price_base: BackTestPriceBase,
  ) -> GenericResult<impl Stream<Item = Price> + '_> {
    let half_spread = self.spread / 2.0;
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
        return Price {
          symbol: kline.symbol.clone(),
          ask: price + half_spread,
          bid: price - half_spread,
          asset_volume: kline.volume,
          base_volume: kline.quote_volume,
          price_base,
        };
      })
      .boxed();
    self.cur_trade = None;
    return Ok(stream! {
      while let Some(v) = stream.next().await {
        self.cur_trade = Some(v.clone());
        yield v;
      }
      self.cur_trade = None;
    });
  }

  fn execute_order(&mut self) -> GenericResult<()> {
    if self.cur_trade.is_none() {
      return Err(Box::new(ExecutionFailed::new(
        "Trade Stream seems to be closed.",
      )));
    }
    let cur_trade = self.cur_trade.clone().unwrap();
    for (key, orders) in self.orders.iter_mut() {
      let position = orders
        .into_iter()
        .filter(|order| order.price >= cur_trade.ask)
        .fold(Order::default(), |mut acc, order| {
          acc += order;
          return acc;
        });
      let remain = orders
        .into_iter()
        .filter(|order| order.price < cur_trade.ask);
      match self.positions.get_mut(key) {
        None => {
          self.positions.insert(key.clone(), position);
        }
        Some(v) => {
          v.price = (position.price * position.qty) + (v.price * v.qty)
        }
      }
    }
    return Ok(());
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
    if self.cur_trade.is_none() {
      return Err(Box::new(ExecutionFailed::new(
        "Trade Stream seems to be closed.",
      )));
    }
    let id = ObjectId::new();
    let price = price.unwrap_or(self.cur_trade.as_ref().unwrap().ask);
    let orders = match order_option {
      None => vec![Order {
        symbol: symbol.clone(),
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
            Order {
              symbol: symbol.clone(),
              price: order_price.clone(),
              qty: amount / order_price,
            }
          })
          .collect()
      }
    };
    return Ok(id);
  }
  async fn remove_order(&self, id: ObjectId) -> GenericResult<ExecutionResult> {
    return Ok(ExecutionResult::default());
  }
}
