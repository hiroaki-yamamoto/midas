use ::async_trait::async_trait;

use ::errors::SymbolFetchResult;

use super::entities::Symbol;

#[async_trait]
pub trait SymbolFetcher {
  type SymbolType: Symbol;
  async fn refresh(&mut self) -> SymbolFetchResult<Vec<Self::SymbolType>>;
}
