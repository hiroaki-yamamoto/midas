use ::async_trait::async_trait;

use ::types::ThreadSafeResult;

use super::entities::Symbol;

#[async_trait]
pub trait SymbolFetcher {
  type SymbolType: Symbol;
  async fn refresh(&self) -> ThreadSafeResult<Vec<Self::SymbolType>>;
}
