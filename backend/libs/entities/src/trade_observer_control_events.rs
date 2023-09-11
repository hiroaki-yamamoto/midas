use ::serde::{Deserialize, Serialize};
use ::uuid::Uuid;

use ::rpc::entities::Exchanges;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverNodeEvent {
  Regist(Exchanges),
  Unregist(Uuid),
  Ping(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverControlEvent {
  /// This enum is sent as a response when the node id is assigned.
  NodeIDAssigned(Uuid),
  /// Triggered when the controller instructs the observer to add a symbol.
  SymbolAdd(Exchanges, String),
  /// Triggered when the controller instructs the observer to remove a symbol.
  SymbolDel(Exchanges, String),
}
