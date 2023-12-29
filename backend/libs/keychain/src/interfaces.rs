use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::mongodb::bson::{oid::ObjectId, Document};
use ::mongodb::error::Result as DBResult;

use ::entities::APIKey;
use ::errors::KeyChainResult;
use ::rpc::exchanges::Exchanges;

#[async_trait]
pub trait IKeyChain {
  async fn push(&self, api_key: APIKey) -> KeyChainResult<Option<ObjectId>>;
  async fn rename_label(&self, id: ObjectId, label: &str)
    -> KeyChainResult<()>;
  async fn list(
    &self,
    filter: Document,
  ) -> KeyChainResult<BoxStream<'_, APIKey>>;
  async fn get(
    &self,
    exchange: Exchanges,
    id: ObjectId,
  ) -> DBResult<Option<APIKey>>;
  async fn delete(&self, id: ObjectId) -> KeyChainResult<()>;
}
