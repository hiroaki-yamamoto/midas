use ::std::collections::HashMap;

use ::async_trait::async_trait;
use ::bytes::Bytes;
use ::chrono::{DateTime, Utc};
use ::futures::stream::{BoxStream, LocalBoxStream, Stream};
use ::mongodb::bson::{doc, oid::ObjectId, Document};
use ::mongodb::results::InsertManyResult;
use ::mongodb::Database;
use ::nats::asynk::Subscription;
use ::ring::hmac;
use ::serde::Serialize;

use ::types::GenericResult;
use types::ThreadSafeResult;

use crate::entities::APIKeyInternal;

use super::entities::{
  BookTicker, ExecutionResult, ExecutionType, Order, OrderInner, OrderOption,
};

use super::errors::ExecutionFailed;

#[async_trait]
pub trait TradeObserver {
  async fn start(&self) -> GenericResult<()>;
  async fn subscribe(&self) -> ::std::io::Result<BoxStream<'_, BookTicker>>;
}

pub(crate) trait TradeDateTime {
  fn symbol(&self) -> String;
  fn open_time(&self) -> DateTime<Utc>;
  fn close_time(&self) -> DateTime<Utc>;
}

#[async_trait]
pub trait Executor {
  async fn open(
    &mut self,
    api_key_id: ObjectId,
  ) -> GenericResult<LocalBoxStream<'_, GenericResult<BookTicker>>>;
  async fn create_order(
    &mut self,
    api_key_id: ObjectId,
    symbol: String,
    price: Option<f64>,
    budget: f64,
    order_option: Option<OrderOption>,
  ) -> GenericResult<ObjectId>;

  async fn remove_order(
    &mut self,
    api_key_id: ObjectId,
    id: ObjectId,
  ) -> GenericResult<ExecutionResult>;
}

pub trait TestExecutor {
  fn get_current_trade(&self) -> Option<BookTicker>;
  fn maker_fee(&self) -> f64;
  fn taker_fee(&self) -> f64;
  fn get_orders(&self) -> HashMap<ObjectId, Order>;
  fn get_positions(&self) -> HashMap<ObjectId, OrderInner>;
  fn set_orders(&mut self, orders: HashMap<ObjectId, Order>);
  fn set_positions(&mut self, positions: HashMap<ObjectId, OrderInner>);
  fn execute_order(
    &mut self,
    exe_type: ExecutionType,
  ) -> Result<(), ExecutionFailed> {
    let cur_trade = self.get_current_trade();
    if cur_trade.is_none() {
      return Err(ExecutionFailed::new("Trade Stream seems to be closed."));
    }
    let cur_trade = cur_trade.unwrap();
    let fee = match exe_type {
      ExecutionType::Maker => self.maker_fee(),
      ExecutionType::Taker => self.taker_fee(),
    };
    let mut positions = self.get_positions();
    let mut orders = self.get_orders();
    for (key, order) in orders.iter_mut() {
      if order.symbol != cur_trade.symbol {
        continue;
      }
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
      match positions.get_mut(key) {
        None => {
          positions.insert(key.clone(), position);
        }
        Some(v) => {
          *v += position;
        }
      }
      order.inner = remain;
    }
    self.set_orders(orders);
    self.set_positions(positions);
    return Ok(());
  }

  fn create_order(
    &mut self,
    symbol: String,
    price: Option<f64>,
    budget: f64,
    order_option: Option<OrderOption>,
  ) -> GenericResult<ObjectId> {
    let cur_trade = self.get_current_trade();
    if cur_trade.is_none() {
      return Err(Box::new(ExecutionFailed::new("Trade Stream is closed.")));
    }
    let id = ObjectId::new();
    let price = price.unwrap_or(cur_trade.as_ref().unwrap().ask_price);
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
    let mut order_dict = self.get_orders();
    order_dict.insert(id.clone(), orders);
    self.set_orders(order_dict);
    self.execute_order(ExecutionType::Maker)?;
    return Ok(id);
  }

  fn remove_order(&mut self, id: ObjectId) -> GenericResult<ExecutionResult> {
    let trade = self.get_current_trade();
    if trade.is_none() {
      return Err(Box::new(ExecutionFailed::new("Trade stream is closed.")));
    }
    let cur_trade = trade.unwrap();
    let price = cur_trade.bid_price;
    let mut orders = self.get_orders();
    let mut positions = self.get_positions();
    let fee = self.taker_fee();
    let ret = match positions.get_mut(&id) {
      None => ExecutionResult::default(),
      Some(v) => {
        let qty = v.qty * (1.0 - fee);
        let profit = ((qty * price) - (v.qty * v.price)) / qty;
        let profit_ratio = profit / v.price;
        ExecutionResult {
          id: id.clone(),
          price,
          profit,
          profit_ratio,
          qty: v.qty,
        }
      }
    };
    orders.remove(&id);
    positions.remove(&id);
    self.set_positions(positions);
    self.set_orders(orders);
    return Ok(ret);
  }
}

pub trait Sign {
  fn get_secret_key(&self, prv_key: String) -> hmac::Key;
  fn sign(&self, body: String, prv_key: String) -> String {
    let secret = self.get_secret_key(prv_key);
    let tag = hmac::sign(&secret, body.as_bytes());
    let signature = Bytes::copy_from_slice(tag.as_ref());
    return format!("{:x}", signature);
  }
}

#[async_trait]
pub trait UserStream {
  async fn get_listen_key(&self, api_key: &APIKeyInternal)
    -> GenericResult<()>;
  async fn clise_listen_key(
    &self,
    api_key: &APIKeyInternal,
    listen_key: &String,
  ) -> GenericResult<()>;
  async fn start(&self) -> GenericResult<()>;
}
