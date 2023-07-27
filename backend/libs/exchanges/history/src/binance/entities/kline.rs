use ::std::time::SystemTime;

use ::mongodb::bson::DateTime as MongoDateTime;
use ::serde::{Deserialize, Serialize};
use ::serde_json::Value;

use ::entities::{TradeTime, TradeTimeTrait};
use ::errors::ParseResult;
use ::rug::{Float, Integer};
use ::types::casting::{cast_datetime, cast_f, cast_i64};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
  pub symbol: String,
  pub open_time: MongoDateTime,
  pub open_price: Float,
  pub high_price: Float,
  pub low_price: Float,
  pub close_price: Float,
  pub volume: Float,
  pub close_time: MongoDateTime,
  pub quote_volume: Float,
  pub num_trades: i64,
  pub taker_buy_base_volume: Float,
  pub taker_buy_quote_volume: Float,
}

impl Kline {
  pub fn new(symbol: String, payload: &Vec<Value>) -> ParseResult<Self> {
    return Ok(Kline {
      symbol,
      open_time: cast_datetime("open_time", &payload[0])?,
      open_price: cast_f("open_price", &payload[1])?,
      high_price: cast_f("high_price", &payload[2])?,
      low_price: cast_f("low_price", &payload[3])?,
      close_price: cast_f("close_price", &payload[4])?,
      volume: cast_f("volume", &payload[5])?,
      close_time: cast_datetime("close_time", &payload[6])?,
      quote_volume: cast_f("quote_volume", &payload[7])?,
      num_trades: cast_i64("num_trades", &payload[8])?,
      taker_buy_base_volume: cast_f("taker_buy_base_volume", &payload[9])?,
      taker_buy_quote_volume: cast_f("taker_buy_quote_volume", &payload[10])?,
    });
  }
}

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
