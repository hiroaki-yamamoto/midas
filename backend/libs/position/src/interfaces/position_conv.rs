use ::async_trait::async_trait;

use ::errors::PositionResult;
use ::rpc::position::Position as PositionRpc;

use crate::entities::Position;

#[async_trait]
pub trait IPositionConverter {
  async fn to_rpc(&self, position: &Position) -> PositionResult<PositionRpc>;
}
