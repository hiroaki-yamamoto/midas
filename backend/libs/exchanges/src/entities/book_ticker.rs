use ::serde::{Deserialize, Serialize};

use ::rpc::bookticker::BookTicker as RPCBookTicker;
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

impl From<BookTicker> for RPCBookTicker {
  fn from(me: BookTicker) -> Self {
    return Self {
      id: me.id,
      symbol: me.symbol,
      bid_price: me.bid_price,
      bid_qty: me.bid_qty,
      ask_price: me.ask_price,
      ask_qty: me.ask_qty,
    };
  }
}
