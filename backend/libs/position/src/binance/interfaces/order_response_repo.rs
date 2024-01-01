use ::async_trait::async_trait;
use ::errors::PositionResult;
use ::mongodb::bson::DateTime;
use ::mongodb::results::UpdateResult;
use ::rug::Float;

use super::super::entities::OrderResponse;

#[async_trait]
pub trait IOrderResponseRepo {
  async fn save(
    &self,
    order_responses: &[OrderResponse<Float, DateTime>],
  ) -> PositionResult<UpdateResult>;
}
