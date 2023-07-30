use super::OrderInner;
use ::bson::oid::ObjectId;
use ::rug::Float;
use ::types::stateful_setter;

#[derive(Clone, Debug)]
pub enum ExecutionType {
  Maker,
  Taker,
}

#[derive(Debug, Clone, Default)]
pub struct ExecutionSummary {
  pub id: Option<ObjectId>,
  pub price: Option<Float>,
  pub qty: Option<Float>,
  pub profit: Option<Float>,
  pub profit_ratio: Option<Float>,
}

impl ExecutionSummary {
  stateful_setter!(id, Option<ObjectId>);
  stateful_setter!(price, Option<Float>);
  stateful_setter!(qty, Option<Float>);
  stateful_setter!(profit, Option<Float>);
  stateful_setter!(profit_ratio, Option<Float>);

  pub fn calculate_profit(op_a: &OrderInner, op_b: &OrderInner) -> Self {
    let profit = op_a.price.clone() - &op_b.price;
    let profit_ratio =
      op_a.qty.clone() * &op_a.price / op_b.qty.clone() * &op_b.price - 1.0;
    return Self::default()
      .price(Some(op_a.price.clone()))
      .qty(Some(op_a.qty.clone()))
      .profit(Some(profit))
      .profit_ratio(Some(profit_ratio))
      .clone();
  }
}
