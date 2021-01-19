use ::std::str::FromStr;

use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::DateTime;
use ::serde::{Deserialize, Serialize};

use super::order::OrderStatus;
use super::side::Side;

use crate::entities::{Order as CommonOrder, OrderInner as CommonOrderInner};
use crate::errors::ParseError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionType {
  New,
  Canceled,
  Replaced,
  Rejected,
  Trade,
  Expired,
}

impl FromStr for ExecutionType {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_uppercase().as_str() {
      "NEW" => {
        return Ok(Self::New);
      }
      "CANCELED" => {
        return Ok(Self::Canceled);
      }
      "REPLACED" => {
        return Ok(Self::Replaced);
      }
      "REJECTED" => {
        return Ok(Self::Rejected);
      }
      "TRADE" => {
        return Ok(Self::Trade);
      }
      "EXPIRED" => {
        return Ok(Self::Expired);
      }
      _ => {
        return Err(ParseError::new(s.to_string()));
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport<FloatType, DateTimeType> {
  #[serde(rename = "s")]
  symbol: String,
  #[serde(rename = "i")]
  order_id: u64,
  #[serde(rename = "c")]
  client_order_id: String,
  #[serde(rename = "S")]
  side: Side,
  #[serde(rename = "x")]
  exec_type: ExecutionType,
  #[serde(rename = "X")]
  order_status: OrderStatus,
  #[serde(rename = "r")]
  reject_reason: String,
  #[serde(rename = "l")]
  executed_qty: FloatType,
  #[serde(rename = "z")]
  acc_qty: FloatType,
  #[serde(rename = "L")]
  price: FloatType,
  #[serde(rename = "n")]
  commission_amount: FloatType,
  #[serde(rename = "N")]
  commission_asset: Option<String>,
  #[serde(rename = "T")]
  trade_time: DateTimeType,
  #[serde(rename = "t")]
  trade_id: i64,
}

impl From<ExecutionReport<f64, DateTime>> for CommonOrderInner {
  fn from(value: ExecutionReport<f64, DateTime>) -> Self {
    return Self {
      price: value.price,
      qty: value.executed_qty,
    };
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionReports {
  #[serde(rename = "_id")]
  id: ObjectId,
  symbol: String,
  reports: Vec<ExecutionReport<f64, DateTime>>,
}

impl From<ExecutionReports> for CommonOrder {
  fn from(value: ExecutionReports) -> Self {
    let traded: Vec<CommonOrderInner> = value
      .reports
      .into_iter()
      .filter(|report| report.exec_type == ExecutionType::Trade)
      .map(|report| report.into())
      .collect();
    return Self {
      symbol: value.symbol,
      inner: traded,
    };
  }
}
