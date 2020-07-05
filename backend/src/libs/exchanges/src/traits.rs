use ::async_trait::async_trait;
use ::tokio::sync::mpsc::Receiver;
use ::reqwest::IntoUrl;
use ::rpc::historical::HistChartProg;
use ::rpc::entities::SymbolInfo;
use ::url::Url;

#[async_trait]
trait Exchange {
  async fn get_ws_endpoint(&self) -> Url;
  async fn get_rest_endpoint<T>(&self) -> T where T: IntoUrl;
  async fn refresh_historical(&self, symbol: String) -> Receiver<HistChartProg>;
  async fn refresh_symbols(&self) -> Receiver<Vec<SymbolInfo>>;
}
