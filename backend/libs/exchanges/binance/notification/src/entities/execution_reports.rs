use ::std::str::FromStr;
use std::num::ParseFloatError;

use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::DateTime;
use ::serde::{Deserialize, Serialize};

use ::binance_executors::entities::{OrderStatus, Side};
use ::entities::{Order as CommonOrder, OrderInner as CommonOrderInner};

use ::errors::ParseError;
use ::types::casting::cast_datetime_from_i64;

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
pub struct ExecutionReport<DateTimeType, FloatType> {
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

impl From<ExecutionReport<DateTime, f64>> for CommonOrderInner {
  fn from(value: ExecutionReport<DateTime, f64>) -> Self {
    return Self {
      price: value.price,
      qty: value.executed_qty,
    };
  }
}

impl From<ExecutionReport<i64, String>>
  for Result<ExecutionReport<DateTime, f64>, ParseFloatError>
{
  fn from(v: ExecutionReport<i64, String>) -> Self {
    return Ok(ExecutionReport::<DateTime, f64> {
      symbol: v.symbol,
      order_id: v.order_id,
      client_order_id: v.client_order_id,
      side: v.side,
      exec_type: v.exec_type,
      order_status: v.order_status,
      reject_reason: v.reject_reason,
      executed_qty: v.executed_qty.parse()?,
      acc_qty: v.acc_qty.parse()?,
      price: v.price.parse()?,
      commission_amount: v.commission_amount.parse()?,
      commission_asset: v.commission_asset,
      trade_time: cast_datetime_from_i64(v.trade_time).into(),
      trade_id: v.trade_id,
    });
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionReports {
  #[serde(rename = "_id")]
  id: ObjectId,
  symbol: String,
  reports: Vec<ExecutionReport<DateTime, f64>>,
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
