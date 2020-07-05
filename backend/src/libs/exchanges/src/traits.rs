use ::async_trait::async_trait;
use ::tokio::sync::mpsc::Receiver;
use ::rpc::historical::HistChartProg;
use ::rpc::entities::SymbolInfo;

#[async_trait]
pub trait Exchange {
  async fn refresh_historical(&self, symbol: String) -> Receiver<HistChartProg>;
  async fn refresh_symbols(&self) -> Receiver<Vec<SymbolInfo>>;
}
