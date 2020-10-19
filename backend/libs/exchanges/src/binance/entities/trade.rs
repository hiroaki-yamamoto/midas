use ::mongodb::bson::DateTime as MongoDateTime;
use ::serde::{Deserialize, Serialize};
use ::serde_json::Value;

use ::types::SendableErrorResult;

use crate::casting::{cast_datetime, cast_f64};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TradeSubRequestInner{
  pub id: u32,
  pub params: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "method")]
pub(crate) enum TradeSubRequest {
  #[serde(rename = "SUBSCRIBE")]
  Subscribe(TradeSubRequestInner),
  #[serde(rename = "UNSUBSCRIBE")]
  Unsubscribe(TradeSubRequestInner),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TradeRaw {
  #[serde(rename = "E")]
  event_time: Value,
  #[serde(rename = "s")]
  symbol: String,
  #[serde(rename = "t")]
  trade_id: i64,
  #[serde(rename = "p")]
  price: Value,
  #[serde(rename = "q")]
  quantity: Value,
  #[serde(rename = "b")]
  buyer_order_id: i64,
  #[serde(rename = "a")]
  seller_order_id: i64,
  #[serde(rename = "T")]
  trade_time: Value,
  #[serde(rename = "m")]
  trade_type: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum TradeType {
  Sell, // Is the buyer the market maker? -> True
  Buy,  // Is the buyer the market maker? -> False
}

impl From<bool> for TradeType {
  fn from(is_sell: bool) -> Self {
    return match is_sell {
      true => Self::Sell,
      false => Self::Buy,
    };
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Trade {
  pub event_time: MongoDateTime,
  pub symbol: String,
  pub trade_id: String,
  pub price: f64,
  pub quantity: f64,
  pub buyer_order_id: String,
  pub seller_order_id: String,
  pub trade_time: MongoDateTime,
  pub trade_type: TradeType,
}

impl From<TradeRaw> for SendableErrorResult<Trade> {
  fn from(data: TradeRaw) -> Self {
    return Ok(Trade {
      event_time: cast_datetime("event_time", &data.event_time)?.into(),
      symbol: data.symbol,
      trade_id: data.trade_id.to_string(),
      price: cast_f64("price", &data.price)?,
      quantity: cast_f64("quantity", &data.quantity)?,
      buyer_order_id: data.buyer_order_id.to_string(),
      seller_order_id: data.seller_order_id.to_string(),
      trade_time: cast_datetime("trade_time", &data.trade_time)?.into(),
      trade_type: data.trade_type.into(),
    });
  }
}
