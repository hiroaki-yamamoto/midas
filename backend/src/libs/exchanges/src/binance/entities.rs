use ::chrono::{DateTime, Utc};
use ::serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HistQuery {
  pub symbol: String,
  pub interval: String,
  pub start_time: String,
  pub end_time: String,
  pub limit: String,
}

pub struct HistFetcherParam {
  pub symbol: String,
  pub start_time: DateTime<Utc>,
  pub end_time: DateTime<Utc>,
}

pub struct Kline {
  pub open_time: DateTime<Utc>,
  pub open_price: f64,
  pub high_price: f64,
  pub low_price: f64,
  pub close_price: f64,
  pub volume: f64,
  pub close_time: DateTime<Utc>,
  pub quote_volume: f64,
  pub num_trades: i64,
  pub taker_buy_base_volume: f64,
  pub taker_buy_quote_volume: f64,
}
