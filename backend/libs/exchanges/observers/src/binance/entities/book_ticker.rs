use ::std::convert::TryFrom;

use ::entities::BookTicker as CommonBookTicker;
use ::errors::ParseError;
use ::rpc::entities::Exchanges;
use ::rug::Float;
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

impl TryFrom<BookTicker<String>> for BookTicker<Float> {
  type Error = ParseError;

  fn try_from(value: BookTicker<String>) -> Result<Self, Self::Error> {
    let bid_price = Float::parse(&value.bid_price)
      .map_err(Self::Error::raise_parse_err("bid_price", value.bid_price))?;
    let bid_qty = Float::parse(&value.bid_qty)
      .map_err(Self::Error::raise_parse_err("bid_qty", value.bid_qty))?;
    let ask_price = Float::parse(&value.ask_price)
      .map_err(Self::Error::raise_parse_err("ask_price", value.ask_price))?;
    let ask_qty = Float::parse(&value.ask_qty)
      .map_err(Self::Error::raise_parse_err("ask_qty", value.ask_qty))?;

    let bid_price = Float::with_val(32, bid_price);
    let bid_qty = Float::with_val(32, bid_qty);
    let ask_price = Float::with_val(32, ask_price);
    let ask_qty = Float::with_val(32, ask_qty);
    return Ok(Self {
      id: value.id,
      symbol: value.symbol,
      bid_price,
      bid_qty,
      ask_price,
      ask_qty,
    });
  }
}

impl From<&BookTicker<Float>> for CommonBookTicker {
  fn from(value: &BookTicker<Float>) -> Self {
    return Self {
      exchange: Exchanges::Binance,
      symbol: value.symbol.clone(),
      id: value.id.to_string(),
      bid_price: value.bid_price.clone(),
      bid_qty: value.bid_qty.clone(),
      ask_price: value.ask_price.clone(),
      ask_qty: value.ask_qty.clone(),
    };
  }
}

impl From<BookTicker<Float>> for CommonBookTicker {
  fn from(value: BookTicker<Float>) -> Self {
    return (&value).into();
  }
}
