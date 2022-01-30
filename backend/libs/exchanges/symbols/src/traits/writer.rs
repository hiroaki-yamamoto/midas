use ::async_trait::async_trait;
use ::futures::Stream;
use ::mongodb::bson::Document;
use ::mongodb::results::InsertManyResult;
use ::serde::Serialize;

use super::entities::Symbol as SymbolTrait;
use ::types::ThreadSafeResult;

#[async_trait]
pub trait SymbolRecorder {
  type Type: SymbolTrait + Serialize + Send + 'static;
  type ListStream: Stream<Item = Self::Type> + Send + 'static;
  async fn list(
    &self,
    query: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> ThreadSafeResult<Self::ListStream>;
  async fn update_symbols(
    &self,
    value: Vec<Self::Type>,
  ) -> ThreadSafeResult<InsertManyResult>;
  async fn list_base_currencies(&self) -> ThreadSafeResult<Vec<String>>;
}
