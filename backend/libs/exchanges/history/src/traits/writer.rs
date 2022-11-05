use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::mongodb::bson::Document;
use ::mongodb::error::Result as MongoResult;
use ::mongodb::results::{DeleteResult, InsertManyResult};
use ::types::ThreadSafeResult;

use crate::entities::KlinesByExchange;

#[async_trait]
pub trait HistoryWriter {
  async fn delete_by_symbol(&self, symbol: &str) -> MongoResult<DeleteResult>;
  async fn write(
    &self,
    klines: KlinesByExchange,
  ) -> ThreadSafeResult<InsertManyResult>;
  async fn list(
    self,
    query: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> MongoResult<BoxStream<'async_trait, KlinesByExchange>>;
}
