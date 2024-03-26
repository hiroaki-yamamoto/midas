use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::results::UpdateResult;

use ::errors::PositionResult;
use ::rpc::pagination::Pagination;

use crate::entities::Position;

#[async_trait]
pub trait IPositionRepo {
  async fn save(&self, position: &[&Position]) -> PositionResult<UpdateResult>;
  async fn get(&self, id: &ObjectId) -> PositionResult<Position>;
  async fn list_by_bot_id(
    &self,
    bot_id: ObjectId,
    pg: Pagination,
  ) -> PositionResult<BoxStream<'_, PositionResult<Position>>>;
}
