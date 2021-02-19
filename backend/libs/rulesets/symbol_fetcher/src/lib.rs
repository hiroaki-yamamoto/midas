use ::async_trait::async_trait;

use ::types::ThreadSafeResult;

#[async_trait]
pub trait SymbolFetcher {
  async fn refresh(&self) -> ThreadSafeResult<()>;
}
