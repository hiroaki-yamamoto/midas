use ::async_trait::async_trait;
use ::nats::asynk::Subscription;
use ::types::SendableErrorResult;

#[async_trait]
pub trait HistoryFetcher {
  async fn refresh(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<Subscription>;
  async fn stop(&self) -> SendableErrorResult<()>;
}

#[async_trait]
pub trait SymbolFetcher {
  async fn refresh(&self) -> SendableErrorResult<()>;
}
