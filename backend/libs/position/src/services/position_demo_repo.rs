use ::async_trait::async_trait;

use ::errors::PositionResult;

use crate::entities::Position;
use crate::interfaces::IPositionRepo;

pub struct PositionDemoRepo;

#[async_trait]
impl IPositionRepo for PositionDemoRepo {
  async fn save(&self, position: &Position) -> PositionResult<Position> {
    return Ok(position.clone());
  }
}
