use ::rug::Float;
use ::serde::{Deserialize, Serialize};

use ::rpc::bookticker::Bookticker as RPCBookTicker;
use ::rpc::exchange::Exchange;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookTicker {
  pub exchange: Exchange,
  pub id: String,
  pub symbol: String,
  pub bid_price: Float,
  pub bid_qty: Float,
  pub ask_price: Float,
  pub ask_qty: Float,
}

impl From<BookTicker> for RPCBookTicker {
  fn from(me: BookTicker) -> Self {
    return Self {
      id: me.id,
      symbol: me.symbol,
      bid_price: me.bid_price.to_string(),
      bid_qty: me.bid_qty.to_string(),
      ask_price: me.ask_price.to_string(),
      ask_qty: me.ask_qty.to_string(),
    };
  }
}
