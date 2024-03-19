use ::async_trait::async_trait;

use ::errors::PositionResult;
use ::rpc::position::Position as PositionRpc;

use crate::entities::Position;

#[async_trait]
pub trait IPositionRpcConv {
  async fn to_rpc(&self, position: &Position) -> PositionResult<PositionRpc>;
  async fn from_rpc(
    &self,
    position_rpc: &PositionRpc,
  ) -> PositionResult<Position>;
}
