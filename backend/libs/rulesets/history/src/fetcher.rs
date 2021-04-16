use ::async_trait::async_trait;

use ::types::ThreadSafeResult;

#[async_trait]
pub trait HistoryFetcher {
  async fn refresh(&self, symbols: Vec<String>) -> ThreadSafeResult<()>;
  async fn stop(&self) -> ThreadSafeResult<()>;
  async fn spawn(&self) -> ThreadSafeResult<()>;
}
