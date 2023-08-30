use ::async_trait::async_trait;
use ::futures::Stream;
use ::mongodb::bson::Document;
use ::mongodb::error::Result as DBResult;

use ::rpc::symbols::SymbolInfo;

#[async_trait]
pub trait SymbolWriter {
  type ListStream: Stream<Item = SymbolInfo> + Send + 'static;
  async fn list(
    &self,
    query: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> DBResult<Self::ListStream>; // will remove
  async fn list_trading(&self) -> DBResult<Self::ListStream>;
  async fn list_base_currencies(&self) -> DBResult<Vec<String>>;
}
