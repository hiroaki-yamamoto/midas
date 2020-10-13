use ::serde::{Deserialize, Serialize};

use super::trade::TradeRaw;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e")]
pub(crate) enum StreamEvent {
  Trade(TradeRaw),
}
