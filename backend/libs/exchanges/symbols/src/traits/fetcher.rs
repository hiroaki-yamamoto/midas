use ::async_trait::async_trait;

use ::errors::SymbolFetchResult;

use ::rpc::symbols::SymbolInfo;

#[async_trait]
pub trait SymbolFetcher {
  async fn refresh(&mut self) -> SymbolFetchResult<Vec<SymbolInfo>>;
}
