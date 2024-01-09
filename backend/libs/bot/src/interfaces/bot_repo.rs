use ::async_trait::async_trait;
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::results::UpdateResult;

use crate::entities::Bot;
use crate::errors::BotInfoResult;

#[async_trait]
pub trait IBotRepo {
  async fn get_by_id(&self, id: ObjectId) -> BotInfoResult<Bot>;
  async fn save(&self, model: &[&Bot]) -> BotInfoResult<UpdateResult>;
}
