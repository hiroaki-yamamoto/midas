use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::mongodb::bson::oid::ObjectId;

use ::errors::PositionResult;
use ::rpc::pagination::Pagination;

use crate::entities::Position;

#[async_trait]
pub trait IPositionRepo {
  async fn save(&self, position: &Position) -> PositionResult<Position>;
  async fn get(&self, id: &ObjectId) -> PositionResult<Position>;
  async fn list_by_bot_id(
    &self,
    bot_id: ObjectId,
    pg: Pagination,
  ) -> PositionResult<BoxStream<'_, PositionResult<Position>>>;
}
