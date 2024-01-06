use ::mongodb::bson::DateTime;
use ::rug::Float;

use ::errors::{ExecutionErrors, ExecutionResult};
use ::position::binance::entities::OrderResponse;

use super::super::{
  entities::CancelOrderRequest, interfaces::ICancelOrderRequestMaker,
};

pub struct RequestMaker;

impl RequestMaker {
  pub fn new() -> Self {
    return Self {};
  }
}

impl ICancelOrderRequestMaker for RequestMaker {
  fn build(
    &self,
    order: &OrderResponse<Float, DateTime>,
  ) -> ExecutionResult<CancelOrderRequest<i64>> {
    if order.check_filled() {
      return Err(ExecutionErrors::OrderFilled);
    }
    let cancel_order = CancelOrderRequest::<i64>::new(order.symbol.clone())
      .order_id(Some(order.order_id));
    return Ok(cancel_order);
  }
}
