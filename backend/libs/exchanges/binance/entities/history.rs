use ::std::convert::AsRef;

use ::mongodb::bson::DateTime as MongoDateTime;
use ::serde::{Deserialize, Serialize};
use ::serde_json::Value;

use ::types::{DateTime as ChronoDateTime, ThreadSafeResult};

use crate::casting::{cast_datetime, cast_f64, cast_i64};
use crate::traits::TradeDateTime;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistQuery {
  pub symbol: String,
  pub interval: String,
  pub start_time: String,
  pub end_time: Option<String>,
  pub limit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistFetcherParam {
  pub symbol: String,
  pub num_symbols: i64,
  pub entire_data_len: i64,
  pub start_time: MongoDateTime,
  pub end_time: Option<MongoDateTime>,
}

impl AsRef<HistFetcherParam> for HistFetcherParam {
  fn as_ref(&self) -> &Self {
    return self;
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
  pub symbol: String,
  pub open_time: MongoDateTime,
  pub open_price: f64,
  pub high_price: f64,
  pub low_price: f64,
  pub close_price: f64,
  pub volume: f64,
  pub close_time: MongoDateTime,
  pub quote_volume: f64,
  pub num_trades: i64,
  pub taker_buy_base_volume: f64,
  pub taker_buy_quote_volume: f64,
}

impl Kline {
  pub fn new(symbol: String, payload: &Vec<Value>) -> ThreadSafeResult<Self> {
    return Ok(Kline {
      symbol,
      open_time: (cast_datetime("open_time", &payload[0])?).into(),
      open_price: cast_f64("open_price", &payload[1])?,
      high_price: cast_f64("high_price", &payload[2])?,
      low_price: cast_f64("low_price", &payload[3])?,
      close_price: cast_f64("close_price", &payload[4])?,
      volume: cast_f64("volume", &payload[5])?,
      close_time: (cast_datetime("close_time", &payload[6])?).into(),
      quote_volume: cast_f64("quote_volume", &payload[7])?,
      num_trades: cast_i64("num_trades", &payload[8])?,
      taker_buy_base_volume: cast_f64("taker_buy_base_volume", &payload[9])?,
      taker_buy_quote_volume: cast_f64("taker_buy_quote_volume", &payload[10])?,
    });
  }
}

pub type Klines = Vec<Kline>;

impl AsRef<Kline> for Kline {
  fn as_ref(&self) -> &Self {
    return self;
  }
}

impl TradeDateTime for Kline {
  fn open_time(&self) -> ChronoDateTime {
    return *self.open_time;
  }
  fn close_time(&self) -> ChronoDateTime {
    return *self.close_time;
  }
  fn symbol(&self) -> String {
    return self.symbol.clone();
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlinesWithInfo {
  pub symbol: String,
  pub num_symbols: i64,
  pub entire_data_len: i64,
  pub klines: Klines,
}

impl AsRef<KlinesWithInfo> for KlinesWithInfo {
  fn as_ref(&self) -> &Self {
    return self;
  }
}
