use ::serde::{Deserialize, Serialize};

use ::rpc::symbols::SymbolInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum SymbolEvent {
  Add(SymbolInfo),
  Remove(SymbolInfo),
}
