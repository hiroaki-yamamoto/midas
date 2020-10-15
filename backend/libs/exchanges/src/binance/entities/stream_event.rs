use ::serde::{Deserialize, Serialize};

use super::trade::TradeRaw;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e", rename_all = "camelCase")]
pub(crate) enum StreamEvent {
  Trade(TradeRaw),
}
