use ::serde::{Deserialize, Serialize};

use ::rpc::exchanges::Exchanges;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchStatusChanged {
  pub exchange: Exchanges,
  pub symbol: String,
}
