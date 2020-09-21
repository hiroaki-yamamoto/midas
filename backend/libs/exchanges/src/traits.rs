use ::async_trait::async_trait;
use ::rpc::historical::HistChartProg;
use ::tokio::sync::mpsc;
use ::types::SendableErrorResult;

#[async_trait]
pub trait HistoryFetcher {
  async fn refresh(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<
    mpsc::UnboundedReceiver<SendableErrorResult<HistChartProg>>,
  >;
  async fn stop(&self) -> SendableErrorResult<()>;
}

#[async_trait]
pub trait SymbolFetcher {
  async fn refresh(&self) -> SendableErrorResult<()>;
}
