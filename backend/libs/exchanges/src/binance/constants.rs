pub const REST_ENDPOINT: &str = "https://api.binance.com";
pub const WS_ENDPOINT: &str = "wss://stream.binance.com:9443/ws";
pub static HIST_FETCHER_PARAM_SUB_NAME: &str = "binance.kline.fetch.param";
pub static HIST_FETCHER_FETCH_RESP_SUB_NAME: &str = "binance.kline.fetch.resp";
pub static HIST_FETCHER_FETCH_PROG_SUB_NAME: &str = "binance.kline.fetch.prog";
pub const HIST_RECORDER_LATEST_TRADE_DATE_SUB_NAME: &str =
  "binance.kline.record.latest";
pub const SYMBOL_ADD_EVENT: &str = "binance.symbol.events.add";
pub const SYMBOL_REMOVE_EVENT: &str = "binance.symbol.events.remove";
pub const SYMBOL_INIT_EVENT: &str = "binance.symbol.events.init";
pub const TRADE_OBSERVER_SUB_NAME: &str = "binance.trades";
