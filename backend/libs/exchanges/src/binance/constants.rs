pub static REST_ENDPOINT: &'static str = "https://api.binance.com";
pub static WS_ENDPOINT: &'static str = "wss://stream.binance.com:9443";
pub static HIST_FETCHER_PARAM_SUB_NAME: &str = "binance.kline.fetch.param";
pub static HIST_FETCHER_FETCH_RESP_SUB_NAME: &str = "binance.kline.fetch.resp";
pub static HIST_FETCHER_FETCH_PROG_SUB_NAME: &str = "binance.kline.fetch.prog";
pub const HIST_RECORDER_LATEST_TRADE_DATE_SUB_NAME: &str =
  "binance.kline.record.latest";
pub const SYMBOL_UPDATE_EVENT: &str = "binance.symbol.events.update";
