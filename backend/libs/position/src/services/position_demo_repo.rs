use ::async_trait::async_trait;
use ::mongodb::bson::oid::ObjectId;

use ::errors::PositionResult;
use ::rpc::bot_mode::BotMode;

use crate::entities::Position;
use crate::interfaces::IPositionRepo;

pub struct PositionDemoRepo;

#[async_trait]
impl IPositionRepo for PositionDemoRepo {
  async fn save(&self, position: &Position) -> PositionResult<Position> {
    return Ok(position.clone());
  }
  async fn get(&self, id: &ObjectId) -> PositionResult<Position> {
    let mut pos = Position::new(ObjectId::new(), BotMode::Live, "BTCUSD");
    pos.id = id.clone();
    return Ok(pos);
  }
}
