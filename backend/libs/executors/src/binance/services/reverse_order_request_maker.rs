use ::errors::{ExecutionErrors, ExecutionResult};
use ::mongodb::bson::DateTime;
use ::rug::{float::Round, Float};

use ::position::binance::entities::{OrderResponse, OrderType, Side};

use super::super::{
  entities::OrderRequest, interfaces::IReverseOrderRequestMaker,
};

pub struct RequestMaker;

impl RequestMaker {
  pub fn new() -> Self {
    return Self;
  }
}

impl IReverseOrderRequestMaker for RequestMaker {
  fn build(
    &self,
    order: &OrderResponse<rug::Float, DateTime>,
  ) -> ExecutionResult<OrderRequest<i64>> {
    let sum_qty = order.sum_filled_qty();
    let qty = Float::with_val(128, &sum_qty * 10000);
    let qty = qty
      .to_integer_round(Round::Down)
      .ok_or(ExecutionErrors::InvalidQty(sum_qty))?
      .0;
    let side = order.side.as_ref().unwrap_or(&Side::Buy).clone();
    let req =
      OrderRequest::<i64>::new(order.symbol.clone(), !side, OrderType::Market)
        .quantity(Some(qty.to_string()));
    return Ok(req);
  }
}
