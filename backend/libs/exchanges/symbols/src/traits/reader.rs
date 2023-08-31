use ::async_trait::async_trait;
use ::futures::stream::Stream;
use ::mongodb::error::Result as DBResult;
use ::rpc::symbols::SymbolInfo;

#[async_trait]
pub trait SymbolReader {
  type ListStream: Stream<Item = SymbolInfo> + Send + 'static;
  async fn list_all(&self) -> DBResult<Self::ListStream>; // will remove
  async fn list_trading(&self) -> DBResult<Self::ListStream>;
  async fn list_base_currencies(&self) -> DBResult<Vec<String>>;
}
