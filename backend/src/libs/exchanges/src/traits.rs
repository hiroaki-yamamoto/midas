use ::async_trait::async_trait;
use ::tokio::sync::{mpsc, oneshot};
use ::rpc::historical::HistChartProg;
use ::rpc::entities::SymbolInfo;

#[async_trait]
pub trait Exchange {
  async fn refresh_historical(self, symbol: String) -> (
    oneshot::Sender<()>,
    mpsc::Receiver<HistChartProg>
  );
  // async fn refresh_symbols(&self) -> Receiver<Vec<SymbolInfo>>;
}
