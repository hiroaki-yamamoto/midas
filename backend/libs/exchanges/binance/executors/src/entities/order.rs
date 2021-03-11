use ::serde::{Deserialize, Serialize};

use super::order_type::OrderType;
use super::resp_type::OrderResponseType;
use super::side::Side;
use super::tif::TimeInForce;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest<DT> {
  pub symbol: String,
  pub side: Side,
  #[serde(rename = "type")]
  pub order_type: OrderType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time_in_force: Option<TimeInForce>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quantity: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quote_order_qty: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub price: Option<f64>,
  #[serde(
    rename(serialize = "newClientOrderId", deserialize = "clientOrderId"),
    skip_serializing_if = "Option::is_none"
  )]
  pub client_order_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_price: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iceberg_qty: Option<f64>,
  #[serde(
    rename(serialize = "newOrderRespType", deserialize = "orderRespType"),
    skip_serializing_if = "Option::is_none"
  )]
  pub order_response_type: Option<OrderResponseType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub recv_window: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<DT>,
}
