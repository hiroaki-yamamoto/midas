use std::convert::TryFrom;

use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use types::casting::cast_datetime_from_i64;
use types::errors::ParseError as CastError;

use super::{Fill, OrderType, Side};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse<FT, DT> {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bot_id: Option<ObjectId>,
  pub symbol: String,
  pub order_id: u64,
  pub order_list_id: i64,
  pub client_order_id: String,
  pub transact_time: DT,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub price: Option<FT>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub orig_qty: Option<FT>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub executed_qty: Option<FT>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cummulative_quote_qty: Option<FT>,
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub order_type: Option<OrderType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub side: Option<Side>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fills: Option<Vec<Fill<FT>>>,
}

impl From<OrderResponse<String, i64>>
  for Result<OrderResponse<f64, DateTime>, CastError>
{
  fn from(from: OrderResponse<String, i64>) -> Self {
    return Ok(OrderResponse::<f64, DateTime> {
      id: from.id,
      bot_id: from.bot_id,
      symbol: from.symbol,
      order_id: from.order_id,
      order_list_id: from.order_list_id,
      client_order_id: from.client_order_id,
      transact_time: cast_datetime_from_i64(from.transact_time).into(),
      price: from.price.map(|v| v.parse::<f64>().ok()).flatten(),
      orig_qty: from.orig_qty.map(|v| v.parse::<f64>().ok()).flatten(),
      executed_qty: from.executed_qty.map(|v| v.parse::<f64>().ok()).flatten(),
      cummulative_quote_qty: from
        .cummulative_quote_qty
        .map(|v| v.parse::<f64>().ok())
        .flatten(),
      order_type: from.order_type,
      side: from.side,
      fills: from.fills.map(|v| {
        v.into_iter()
          .filter_map(|item| Fill::<f64>::try_from(item).ok())
          .collect()
      }),
    });
  }
}
