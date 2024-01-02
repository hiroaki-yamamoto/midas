use ::async_trait::async_trait;
use ::errors::PositionResult;
use ::futures::stream::BoxStream;
use ::mongodb::bson::DateTime;
use ::mongodb::results::UpdateResult;
use ::rug::Float;

use crate::entities::Position;

use super::super::entities::OrderResponse;

#[async_trait]
pub trait IOrderResponseRepo {
  async fn save(
    &self,
    order_responses: &[&OrderResponse<Float, DateTime>],
  ) -> PositionResult<UpdateResult>;
  async fn find_by_entry_position(
    &self,
    position: &Position,
  ) -> PositionResult<BoxStream<PositionResult<OrderResponse<Float, DateTime>>>>;
}
