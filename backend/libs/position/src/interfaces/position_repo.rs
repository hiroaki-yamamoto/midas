use ::async_trait::async_trait;
use ::mongodb::results::UpdateResult;

use ::errors::PositionResult;

use crate::entities::Position;

#[async_trait]
pub trait IPositionRepo {
  async fn save(&self, position: &[Position]) -> PositionResult<UpdateResult>;
}
