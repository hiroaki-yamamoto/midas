use ::types::ParseURLResult;

use crate::traits::Exchange;

pub struct Binance;

impl Binance {
  fn get_ws_endpoint(&self) -> ParseURLResult {
    return "wss://stream.binance.com:9443".parse();
  }
  fn get_rest_endpoint(&self) -> ParseURLResult {
    return "https://api.binance.com".parse();
  }
}

impl Exchange for Binance {
  async fn refresh_historical(&self, symbol: String) -> Receiver<HistChartProg> {
  }
  async fn refresh_symbols(&self) -> Receiver<Vec<SymbolInfo>> {
  }
}
