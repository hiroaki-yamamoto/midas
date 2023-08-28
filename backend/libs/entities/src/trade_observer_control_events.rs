use ::serde::{Deserialize, Serialize};
use ::uuid::Uuid;

use ::rpc::entities::Exchanges;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverNodeEvent {
  Regist(Exchanges, Uuid),
  Unregist(Uuid),
  Ping(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverControlEvent {
  // First UUID is the old node ID, second UUID is the new node ID.
  NodeIDChanged(Uuid, Uuid),
  // Triggered when the controller instructs the observer to add a symbol.
  SymbolAdd(Exchanges, String),
  // Triggered when the controller instructs the observer to remove a symbol.
  SymbolDel(Exchanges, String),
}
