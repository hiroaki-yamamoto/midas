use ::serde::{Deserialize, Serialize};
use ::uuid::Uuid;

use ::rpc::symbols::SymbolInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverNodeEvent {
  Regist(Uuid),
  Unregist(Uuid),
  Ping(Uuid, Vec<SymbolInfo>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverControlEvent {
  // First UUID is the old node ID, second UUID is the new node ID.
  NodeIDChanged(Uuid, Uuid),
}
