use super::OrderInner;
use ::bson::oid::ObjectId;
use ::types::stateful_setter;

#[derive(Clone, Debug)]
pub enum ExecutionType {
  Maker,
  Taker,
}

#[derive(Debug, Clone, Default)]
pub struct ExecutionSummary {
  pub id: Option<ObjectId>,
  pub price: Option<f64>,
  pub qty: Option<f64>,
  pub profit: Option<f64>,
  pub profit_ratio: Option<f64>,
}

impl ExecutionSummary {
  stateful_setter!(id, Option<ObjectId>);
  stateful_setter!(price, Option<f64>);
  stateful_setter!(qty, Option<f64>);
  stateful_setter!(profit, Option<f64>);
  stateful_setter!(profit_ratio, Option<f64>);

  pub fn calculate_profit(op_a: &OrderInner, op_b: &OrderInner) -> Self {
    let profit = op_a.price - op_b.price;
    let profit_ratio =
      ((op_a.qty * op_a.price) / (op_b.qty * op_b.price)) - 1.0;
    return Self::default()
      .price(Some(op_a.price))
      .qty(Some(op_a.qty))
      .profit(Some(profit))
      .profit_ratio(Some(profit_ratio))
      .clone();
  }
}
