use ::async_trait::async_trait;
use ::futures_core::stream::BoxStream;
use ::rpc::historical::HistChartProg;

use ::types::ThreadSafeResult;

#[async_trait]
pub trait HistoryFetcher {
  async fn refresh(
    &self,
    symbols: Vec<String>,
  ) -> ThreadSafeResult<BoxStream<HistChartProg>>;
  async fn stop(&self) -> ThreadSafeResult<()>;
  async fn spawn(&self) -> ThreadSafeResult<()>;
}
