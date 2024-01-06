use ::mongodb::bson::DateTime;
use ::rug::Float;

use ::errors::ExecutionResult;
use ::position::binance::entities::OrderResponse;

use super::super::entities::OrderRequest;

pub trait IReverseOrderRequestMaker {
  fn build(
    &self,
    order: &OrderResponse<Float, DateTime>,
  ) -> ExecutionResult<OrderRequest<i64>>;
}
