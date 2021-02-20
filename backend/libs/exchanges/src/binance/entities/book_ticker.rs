use ::std::convert::TryFrom;
use ::std::error::Error;

use ::entities::BookTicker as CommonBookTicker;
use ::rpc::entities::Exchanges;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookTicker<PT> {
  #[serde(rename = "u")]
  id: u64,
  #[serde(rename = "s")]
  symbol: String,
  #[serde(rename = "b")]
  bid_price: PT,
  #[serde(rename = "B")]
  bid_qty: PT,
  #[serde(rename = "a")]
  ask_price: PT,
  #[serde(rename = "A")]
  ask_qty: PT,
}

impl TryFrom<BookTicker<String>> for BookTicker<f64> {
  type Error = Box<dyn Error>;

  fn try_from(value: BookTicker<String>) -> Result<Self, Self::Error> {
    return Ok(Self {
      id: value.id,
      symbol: value.symbol,
      bid_price: value.bid_price.parse()?,
      bid_qty: value.bid_qty.parse()?,
      ask_price: value.ask_price.parse()?,
      ask_qty: value.ask_qty.parse()?,
    });
  }
}

impl From<&BookTicker<f64>> for CommonBookTicker {
  fn from(value: &BookTicker<f64>) -> Self {
    return Self {
      exchange: Exchanges::Binance,
      symbol: value.symbol.clone(),
      id: value.id.to_string(),
      bid_price: value.bid_price,
      bid_qty: value.bid_qty,
      ask_price: value.ask_price,
      ask_qty: value.ask_qty,
    };
  }
}

impl From<BookTicker<f64>> for CommonBookTicker {
  fn from(value: BookTicker<f64>) -> Self {
    return (&value).into();
  }
}
