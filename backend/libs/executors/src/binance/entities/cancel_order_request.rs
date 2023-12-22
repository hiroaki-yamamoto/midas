use ::std::time::SystemTime;

use serde::Serialize;

use ::mongodb::bson::DateTime;
use ::types::stateful_setter;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest<DT> {
  pub symbol: String,
  pub order_id: Option<i64>,
  pub orig_client_order_id: Option<String>,
  pub new_client_order_id: Option<String>,
  pub recv_window: Option<i64>,
  pub timestamp: DT,
}

impl CancelOrderRequest<DateTime> {
  pub fn new(symbol: String) -> Self {
    return Self {
      symbol,
      timestamp: SystemTime::now().into(),
      order_id: None,
      orig_client_order_id: None,
      new_client_order_id: None,
      recv_window: None,
    };
  }
}

impl From<CancelOrderRequest<DateTime>> for CancelOrderRequest<i64> {
  fn from(from: CancelOrderRequest<DateTime>) -> Self {
    return Self {
      symbol: from.symbol,
      order_id: from.order_id,
      orig_client_order_id: from.orig_client_order_id,
      new_client_order_id: from.new_client_order_id,
      recv_window: from.recv_window,
      timestamp: from.timestamp.timestamp_millis(),
    };
  }
}

impl CancelOrderRequest<i64> {
  pub fn new(symbol: String) -> Self {
    return CancelOrderRequest::<DateTime>::new(symbol).into();
  }
}

impl<DT> CancelOrderRequest<DT> {
  stateful_setter!(symbol, String);
  stateful_setter!(order_id, Option<i64>);
  stateful_setter!(orig_client_order_id, Option<String>);
  stateful_setter!(new_client_order_id, Option<String>);
  stateful_setter!(recv_window, Option<i64>);
  stateful_setter!(timestamp, DT);
}
