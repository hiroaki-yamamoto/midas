use ::async_trait::async_trait;
use ::crossbeam::channel::Receiver;
use ::mongodb::bson::Document;
use ::rpc::entities::SymbolInfo;
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
  async fn get(
    &self,
    filter: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> SendableErrorResult<Vec<SymbolInfo>>;
  async fn refresh(self) -> SendableErrorResult<()>;
}
