use ::rug::Float;

use ::errors::ExecutionResult;
use ::mongodb::bson::DateTime;
use ::position::binance::entities::OrderResponse;

use super::super::entities::CancelOrderRequest;

pub trait ICancelOrderRequestMaker {
  fn build(
    &self,
    order: &OrderResponse<Float, DateTime>,
  ) -> ExecutionResult<CancelOrderRequest<i64>>;
}
