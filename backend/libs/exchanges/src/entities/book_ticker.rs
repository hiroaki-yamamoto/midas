use ::serde::{Deserialize, Serialize};

use ::rpc::entities::Exchanges;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookTicker {
  pub exchange: Exchanges,
  pub id: String,
  pub symbol: String,
  pub bid_price: f64,
  pub bid_qty: f64,
  pub ask_price: f64,
  pub ask_qty: f64,
}
