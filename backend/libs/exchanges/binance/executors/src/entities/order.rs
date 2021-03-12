use ::chrono::Utc;
use ::mongodb::bson::DateTime;
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
  pub timestamp: DT,
}

impl OrderRequest<DateTime> {
  pub fn new(symbol: String, side: Side, order_type: OrderType) -> Self {
    return Self {
      symbol,
      side,
      order_type,
      time_in_force: None,
      quantity: None,
      quote_order_qty: None,
      price: None,
      client_order_id: None,
      stop_price: None,
      iceberg_qty: None,
      order_response_type: None,
      recv_window: None,
      timestamp: Utc::now().into(),
    };
  }
}

impl OrderRequest<i64> {
  pub fn new(symbol: String, side: Side, order_type: OrderType) -> Self {
    return OrderRequest::<DateTime>::new(symbol, side, order_type).into();
  }
}

impl From<OrderRequest<DateTime>> for OrderRequest<i64> {
  fn from(v: OrderRequest<DateTime>) -> Self {
    return OrderRequest::<i64> {
      symbol: v.symbol,
      side: v.side,
      order_type: v.order_type,
      time_in_force: v.time_in_force,
      quantity: v.quantity,
      quote_order_qty: v.quote_order_qty,
      price: v.price,
      client_order_id: v.client_order_id,
      stop_price: v.stop_price,
      iceberg_qty: v.iceberg_qty,
      order_response_type: v.order_response_type,
      recv_window: v.recv_window,
      timestamp: v.timestamp.timestamp_millis(),
    };
  }
}
