use ::std::convert::TryFrom;

use ::entities::BookTicker as CommonBookTicker;
use ::errors::ParseError;
use ::rpc::exchange::Exchange;
use ::rug::Float;
use ::serde::{Deserialize, Serialize};
use ::types::casting::cast_f_from_txt;

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
    let bid_price = cast_f_from_txt("bid_price", &value.bid_price)?;
    let bid_qty = cast_f_from_txt("bid_qty", &value.bid_qty)?;
    let ask_price = cast_f_from_txt("ask_price", &value.ask_price)?;
    let ask_qty = cast_f_from_txt("ask_qty", &value.ask_qty)?;

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
      exchange: Exchange::Binance,
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
