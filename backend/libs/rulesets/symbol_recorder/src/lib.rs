use ::async_trait::async_trait;
use ::futures_core::Stream;
use ::mongodb::bson::Document;
use ::mongodb::results::InsertManyResult;
use ::serde::Serialize;

use ::types::ThreadSafeResult;

#[async_trait]
pub trait SymbolRecorder {
  type ListStream: Stream + Send + 'static;
  async fn list(
    &self,
    query: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> ThreadSafeResult<Self::ListStream>;
  async fn quote_assets<T>(&self) -> ThreadSafeResult<T>
  where
    T: Stream<Item = String> + Send + Sync + 'async_trait;
  async fn update_symbols<T>(
    &self,
    value: Vec<T>,
  ) -> ThreadSafeResult<InsertManyResult>
  where
    T: Serialize + Send;
}
