use ::serde::{Deserialize, Serialize};

use ::rpc::symbol_info::SymbolInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum SymbolEvent {
  Add(SymbolInfo),
  Remove(SymbolInfo),
}
