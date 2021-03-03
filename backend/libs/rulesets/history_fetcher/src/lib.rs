use ::async_trait::async_trait;
use ::nats::asynk::Subscription;

use ::types::ThreadSafeResult;

#[async_trait]
pub trait HistoryFetcher {
  async fn refresh(
    &self,
    symbols: Vec<String>,
  ) -> ThreadSafeResult<Subscription>;
  async fn stop(&self) -> ThreadSafeResult<()>;
  async fn spawn(&self) -> ThreadSafeResult<()>;
}
