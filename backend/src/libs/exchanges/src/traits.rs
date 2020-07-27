use ::async_trait::async_trait;
use ::rpc::entities::SymbolInfo;
use ::rpc::historical::HistChartProg;
use ::tokio::sync::{mpsc, oneshot};
use ::types::SendableErrorResult;

#[async_trait]
pub trait Exchange {
  async fn refresh_historical(
    self,
    symbols: Vec<String>,
  ) -> (oneshot::Sender<()>, mpsc::Receiver<HistChartProg>);
  async fn get_symbols(&self) -> SendableErrorResult<Vec<SymbolInfo>>;
}
