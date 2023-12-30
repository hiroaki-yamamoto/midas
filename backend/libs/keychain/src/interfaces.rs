use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::mongodb::bson::{oid::ObjectId, Document};

use ::errors::KeyChainResult;
use ::rpc::exchanges::Exchanges;

use crate::entities::APIKey;

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
  ) -> KeyChainResult<APIKey>;
  async fn delete(&self, id: ObjectId) -> KeyChainResult<()>;
}
