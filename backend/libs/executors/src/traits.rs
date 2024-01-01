use ::std::collections::HashMap;

use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::mongodb::bson::oid::ObjectId;
use ::rug::Float;

use ::entities::{
  BookTicker, ExecutionSummary, ExecutionType, Order, OrderInner, OrderOption,
};
use ::errors::ExecutionFailed;
use ::rpc::bot::Bot;

use ::errors::ExecutionResult;

#[async_trait]
pub trait Executor {
  async fn open(
    &mut self,
  ) -> ExecutionResult<BoxStream<ExecutionResult<BookTicker>>>;

  async fn create_order(
    &mut self,
    bot: &Bot,
    api_key_id: ObjectId,
    symbol: String,
    price: Option<Float>,
    budget: Float,
    order_option: Option<OrderOption>,
  ) -> ExecutionResult<ObjectId>;

  async fn remove_order(
    &mut self,
    bot: &Bot,
    api_key_id: ObjectId,
    id: ObjectId,
  ) -> ExecutionResult<ExecutionSummary>;
}

pub trait TestExecutor {
  fn get_current_trade(&self) -> Option<BookTicker>;
  fn maker_fee(&self) -> Float;
  fn taker_fee(&self) -> Float;
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
          order.qty = order.qty * (1.0 - fee.clone());
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
    price: Option<Float>,
    budget: Float,
    order_option: Option<OrderOption>,
  ) -> ExecutionResult<ObjectId> {
    let cur_trade = self.get_current_trade();
    if cur_trade.is_none() {
      return Err(ExecutionFailed::new("Trade Stream is closed.").into());
    }
    let id = ObjectId::new();
    let price = price.unwrap_or(cur_trade.unwrap().ask_price);
    let orders = match order_option {
      None => vec![OrderInner {
        price: price.clone(),
        qty: budget / price.clone(),
      }],
      Some(v) => v
        .calc_trading_amounts(&budget)
        .into_iter()
        .enumerate()
        .map(|(index, amount)| {
          let order_price = v.calc_order_price(&price, index);
          OrderInner {
            price: order_price.clone(),
            qty: amount / order_price,
          }
        })
        .collect(),
    };
    let orders = Order::new(symbol, orders);
    let mut order_dict = self.get_orders();
    order_dict.insert(id.clone(), orders);
    self.set_orders(order_dict);
    self.execute_order(ExecutionType::Maker)?;
    return Ok(id);
  }

  fn remove_order(
    &mut self,
    id: ObjectId,
  ) -> ExecutionResult<ExecutionSummary> {
    let trade = self.get_current_trade();
    if trade.is_none() {
      return Err(ExecutionFailed::new("Trade stream is closed.").into());
    }
    let cur_trade = trade.unwrap();
    let price = cur_trade.bid_price;
    let mut orders = self.get_orders();
    let mut positions = self.get_positions();
    let fee = self.taker_fee();
    let ret = match positions.get_mut(&id) {
      None => ExecutionSummary::default(),
      Some(v) => {
        let qty = v.qty.clone() * (1.0 - fee);
        let sell_trade = OrderInner { price, qty };
        ExecutionSummary::calculate_profit(&sell_trade, v)
      }
    };
    orders.remove(&id);
    positions.remove(&id);
    self.set_positions(positions);
    self.set_orders(orders);
    return Ok(ret);
  }
}
