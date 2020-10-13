use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::futures::stream::Stream;
use ::nats::asynk::Subscription;
use ::rpc::entities::SymbolInfo;

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
  type ListStream: Stream<Item = SymbolInfo> + Send + 'static;
  async fn refresh(&self) -> SendableErrorResult<()>;
  async fn list(
    &self,
    status: Option<String>,
    symbols: Option<Vec<String>>,
  ) -> SendableErrorResult<Self::ListStream>;
}

#[async_trait]
pub trait HistoryRecorder {
  async fn spawn(&self);
}

#[async_trait]
pub trait TradeObserver {
  async fn start(&self) -> SendableErrorResult<()>;
  async fn subscribe(&self) -> ::std::io::Result<Subscription>;
}

pub(crate) trait TradeDateTime {
  fn symbol(&self) -> String;
  fn open_time(&self) -> DateTime<Utc>;
  fn close_time(&self) -> DateTime<Utc>;
}
