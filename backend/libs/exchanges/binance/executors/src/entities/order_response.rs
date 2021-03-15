use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use types::casting::cast_datetime_from_i64;
use types::errors::ParseError as CastError;

use super::OrderType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse<DT> {
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

impl From<OrderResponse<i64>> for Result<OrderResponse<DateTime>, CastError> {
  fn from(from: OrderResponse<i64>) -> Self {
    return Ok(OrderResponse::<DateTime> {
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
