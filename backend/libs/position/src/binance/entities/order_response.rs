use std::convert::TryFrom;

use log::error;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use rug::{float::Round, Float};
use serde::{Deserialize, Serialize};

use ::entities::Order;
use ::errors::ParseError;
use ::types::casting::{cast_datetime_from_i64, cast_f_from_txt};

use super::{Fill, OrderType, Side};

use crate::interfaces::IProfitCalculable;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse<FT, DT> {
  #[serde(rename = "_id", default = "::mongodb::bson::oid::ObjectId::new")]
  pub id: ObjectId,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gid: Option<ObjectId>,
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
      gid: from.gid,
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

impl<DT> From<&OrderResponse<Float, DT>> for Order {
  fn from(from: &OrderResponse<Float, DT>) -> Self {
    let side = from.side.as_ref();
    let inner = from
      .fills
      .as_ref()
      .map(|fills| {
        fills
          .iter()
          .map(|fill| fill.as_order_inner(side.unwrap_or(&Side::Buy)))
          .collect()
      })
      .unwrap_or(vec![]);
    return Self::new(&from.symbol, &inner);
  }
}

impl OrderResponse<Float, DateTime> {
  pub fn sum_filled_qty(&self) -> Float {
    let sum_fills = match self.fills {
      Some(ref fills) => {
        fills.iter().fold(Float::with_val(128, 0), |acc, fill| {
          return acc + &fill.qty;
        })
      }
      None => Float::with_val(128, 0),
    };
    return sum_fills;
  }

  pub fn check_filled(&self) -> bool {
    if let Some(orig_qty) = self.orig_qty.as_ref() {
      let prec: u32 = 10000;
      let sum_fills =
        (self.sum_filled_qty() * prec).to_integer_round(Round::Down);
      let orig_qty =
        Float::with_val(128, orig_qty * prec).to_integer_round(Round::Down);
      let fills_pair = sum_fills.as_ref().zip(orig_qty.as_ref());
      if let Some(((sum_fills, _), (orig_qty, _))) = fills_pair {
        return sum_fills == orig_qty;
      }
    }
    return false;
  }
}

impl<DT> IProfitCalculable for OrderResponse<Float, DT> {
  fn get_orig_amount(&self) -> Float {
    return self.orig_qty.clone().unwrap_or(Float::with_val(128, 0));
  }
}
