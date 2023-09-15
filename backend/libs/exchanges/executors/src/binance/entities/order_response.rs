use std::convert::TryFrom;

use log::error;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use rug::Float;
use serde::{Deserialize, Serialize};

use ::entities::Order;
use ::errors::ParseError;
use ::types::casting::{cast_datetime_from_i64, cast_f_from_txt};

use super::{Fill, OrderType, Side};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse<FT, DT> {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub position_group_id: Option<ObjectId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub settlement_gid: Option<ObjectId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bot_id: Option<ObjectId>,
  pub symbol: String,
  pub order_id: i64,
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

fn opt_string_to_float(field: &str, v: Option<String>) -> Option<Float> {
  return v
    .map(|v| cast_f_from_txt(field, &v).map_err(|e| error!("{}", e)).ok())
    .flatten();
}

impl TryFrom<OrderResponse<String, i64>> for OrderResponse<Float, DateTime> {
  type Error = ParseError;

  fn try_from(from: OrderResponse<String, i64>) -> Result<Self, Self::Error> {
    return Ok(OrderResponse::<Float, DateTime> {
      id: from.id,
      position_group_id: from.position_group_id,
      settlement_gid: from.settlement_gid,
      bot_id: from.bot_id,
      symbol: from.symbol,
      order_id: from.order_id,
      order_list_id: from.order_list_id,
      client_order_id: from.client_order_id,
      transact_time: cast_datetime_from_i64(from.transact_time).into(),
      price: opt_string_to_float("price", from.price),
      orig_qty: opt_string_to_float("orig_qty", from.orig_qty),
      executed_qty: opt_string_to_float("executed_qty", from.executed_qty),
      cummulative_quote_qty: opt_string_to_float(
        "cummulative_quote_qty",
        from.cummulative_quote_qty,
      ),
      order_type: from.order_type,
      side: from.side,
      fills: from.fills.map(|v| {
        v.into_iter()
          .filter_map(|item| Fill::<Float>::try_from(item).ok())
          .collect()
      }),
    });
  }
}

impl<DT> From<OrderResponse<Float, DT>> for Order {
  fn from(from: OrderResponse<Float, DT>) -> Self {
    let side = from.side;
    let inner = from
      .fills
      .map(|fills| {
        fills
          .clone()
          .into_iter()
          .map(|fill| fill.as_order_inner(side.clone().unwrap_or(Side::Buy)))
          .collect()
      })
      .unwrap_or(vec![]);
    return Self {
      symbol: from.symbol,
      inner,
    };
  }
}
