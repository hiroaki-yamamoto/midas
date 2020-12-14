use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FormatResult};
use ::std::str::FromStr;

use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::DateTime;
use ::serde::{Deserialize, Serialize};

use crate::casting::{cast_datetime_from_i64, ParseError};

#[derive(Debug, Clone)]
pub struct OrderTypeParseFailed {
  pub value: String,
}

impl OrderTypeParseFailed {
  fn new(value: &str) -> Self {
    return Self {
      value: String::from(value),
    };
  }
}

impl Display for OrderTypeParseFailed {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Failed to parse order type: {}", self.value);
  }
}

impl Error for OrderTypeParseFailed {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
  Limit,
  Market,
  StopLoss,
  StopLossLimit,
  TakeProfit,
  TakeProfitLimit,
  LimitMaker,
}

impl FromStr for OrderType {
  type Err = OrderTypeParseFailed;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    return match s {
      "LIMIT" => Ok(Self::Limit),
      "MARKET" => Ok(Self::Market),
      "STOP_LOSS" => Ok(Self::StopLoss),
      "STOP_LOSS_LIMIT" => Ok(Self::StopLossLimit),
      "TAKE_PROFIT" => Ok(Self::TakeProfit),
      "TAKE_PROFIT_LIMIT" => Ok(Self::TakeProfitLimit),
      "LIMIT_MAKER" => Ok(Self::LimitMaker),
      other => Err(OrderTypeParseFailed::new(other)),
    };
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order<DT> {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub bot_id: ObjectId,
  pub symbol: String,
  pub order_id: u64,
  pub order_list_id: i64,
  pub client_order_id: String,
  pub transact_time: DT,
  pub price: f64,
  pub orig_qty: f64,
  pub executed_qty: f64,
  pub cummulative_quote_qty: f64,
  #[serde(rename = "type")]
  pub order_type: OrderType,
}

impl From<Order<i64>> for Result<Order<DateTime>, ParseError> {
  fn from(from: Order<i64>) -> Self {
    return Ok(Order::<DateTime> {
      id: from.id,
      bot_id: from.bot_id,
      symbol: from.symbol,
      order_id: from.order_id,
      order_list_id: from.order_list_id,
      client_order_id: from.client_order_id,
      transact_time: cast_datetime_from_i64(from.transact_time).into(),
      price: from.price,
      orig_qty: from.orig_qty,
      executed_qty: from.executed_qty,
      cummulative_quote_qty: from.cummulative_quote_qty,
      order_type: from.order_type,
    });
  }
}
