use ::serde::{Deserialize, Serialize};

use ::rpc::exchange::Exchange;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchStatusChanged {
  pub exchange: Exchange,
  pub symbol: String,
}
