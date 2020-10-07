use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::nats::asynk::Subscription;
use ::types::SendableErrorResult;

#[async_trait]
pub trait HistoryFetcher {
  async fn refresh(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<Subscription>;
  async fn stop(&self) -> SendableErrorResult<()>;
  async fn spawn(&self) -> SendableErrorResult<()>;
}

#[async_trait]
pub trait SymbolFetcher {
  async fn refresh(&self) -> SendableErrorResult<()>;
}

#[async_trait]
pub trait HistoryRecorder {
  async fn spawn(&self);
}

#[async_trait]
pub trait TradeObserver {
  async fn start(&self) -> SendableErrorResult<Subscription>;
  async fn stop(&self);
}

pub(crate) trait TradeDateTime {
  fn symbol(&self) -> String;
  fn open_time(&self) -> DateTime<Utc>;
  fn close_time(&self) -> DateTime<Utc>;
}
