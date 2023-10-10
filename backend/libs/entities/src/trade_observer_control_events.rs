use ::serde::{Deserialize, Serialize};

use ::rpc::entities::Exchanges;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverNodeEvent {
  Regist(Exchanges),
  Unregist(String),
  Ping(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverControlEvent {
  /// Triggered when the controller instructs the observer to add a symbol.
  SymbolAdd(Exchanges, String),
  /// Triggered when the controller instructs the observer to remove a symbol.
  SymbolDel(Exchanges, String),
}
