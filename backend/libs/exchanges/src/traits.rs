use ::async_trait::async_trait;
use ::crossbeam::channel::Receiver;
use ::rpc::historical::HistChartProg;
use ::types::SendableErrorResult;

#[async_trait]
pub trait HistoryFetcher {
  async fn refresh(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<Receiver<SendableErrorResult<HistChartProg>>>;
  async fn stop(self) -> SendableErrorResult<()>;
}

#[async_trait]
pub trait SymbolFetcher {
  async fn refresh(&self) -> SendableErrorResult<()>;
}
