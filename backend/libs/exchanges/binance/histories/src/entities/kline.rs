use ::std::time::SystemTime;

use ::mongodb::bson::DateTime as MongoDateTime;
use ::serde::{Deserialize, Serialize};
use ::serde_json::Value;

use ::entities::{TradeTime, TradeTimeTrait};
use ::types::casting::{cast_datetime, cast_f64, cast_i64};
use ::types::ThreadSafeResult;

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
      open_time: cast_datetime("open_time", &payload[0])?,
      open_price: cast_f64("open_price", &payload[1])?,
      high_price: cast_f64("high_price", &payload[2])?,
      low_price: cast_f64("low_price", &payload[3])?,
      close_price: cast_f64("close_price", &payload[4])?,
      volume: cast_f64("volume", &payload[5])?,
      close_time: cast_datetime("close_time", &payload[6])?,
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

impl TradeTimeTrait for &Kline {
  fn open_time(&self) -> SystemTime {
    return self.open_time.into();
  }
  fn close_time(&self) -> SystemTime {
    return self.close_time.into();
  }
  fn symbol(&self) -> String {
    return self.symbol.clone();
  }
}

impl TradeTimeTrait for Kline {
  fn open_time(&self) -> SystemTime {
    return self.open_time.into();
  }
  fn close_time(&self) -> SystemTime {
    return self.close_time.into();
  }
  fn symbol(&self) -> String {
    return self.symbol.clone();
  }
}

impl From<Kline> for TradeTime<SystemTime> {
  fn from(kline: Kline) -> Self {
    return Self::from(kline);
  }
}

impl From<&Kline> for TradeTime<SystemTime> {
  fn from(kline: &Kline) -> Self {
    return Self::from(kline);
  }
}

impl From<Kline> for TradeTime<MongoDateTime> {
  fn from(kline: Kline) -> Self {
    return Self::from(kline);
  }
}

impl From<&Kline> for TradeTime<MongoDateTime> {
  fn from(kline: &Kline) -> Self {
    return Self::from(kline);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlinesWithInfo {
  pub symbol: String,
  pub num_symbols: i64,
  pub entire_data_len: u64,
  pub klines: Klines,
}

impl AsRef<KlinesWithInfo> for KlinesWithInfo {
  fn as_ref(&self) -> &Self {
    return self;
  }
}
