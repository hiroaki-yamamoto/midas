use ::async_trait::async_trait;
use ::crossbeam::channel::Receiver;
use ::mongodb::bson::Document;
use ::rpc::entities::SymbolInfo;
use ::rpc::historical::HistChartProg;
use ::types::SendableErrorResult;

#[async_trait]
pub trait Exchange {
  async fn refresh_historical(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<Receiver<SendableErrorResult<HistChartProg>>>;
  async fn get_symbols(
    &self,
    filter: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> SendableErrorResult<Vec<SymbolInfo>>;
  async fn refresh_symbols(self) -> SendableErrorResult<()>;
  async fn stop(self) -> SendableErrorResult<()>;
}
