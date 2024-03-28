use ::async_trait::async_trait;

use crate::entities::Position;
use crate::interfaces::IPositionRepo;

pub struct PositionDemoRepo;

#[async_trait]
impl IPositionRepo for PositionDemoRepo {
  // async fn save(&self, position: &[&Position])
}
