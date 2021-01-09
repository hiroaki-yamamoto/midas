use num::Float;
use ::serde::{Serialize, Deserialize};

use super::order::{OrderStatus, OrderType};
use super::side::Side;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position<FloatType, DateTimeType> {
  #[serde(rename = "s")]
  symbol: String,
  #[serde(rename = "i")]
  order_id: u64,
  #[serde(rename = "c")]
  client_order_id: String,
  #[serde(rename = "S")]
  side: Side,
  #[serde(rename = "x")]
  exec_type: OrderType,
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
