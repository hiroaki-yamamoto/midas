use ::serde::{Deserialize, Serialize};

use ::rpc::exchanges::Exchanges;
use ::symbols::entities::SymbolEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeObserverNodeEvent {
  /// Regist notifies a node is registered/
  Regist(Box<Exchanges>),
  /// Unregist notifies a node is unregistered.
  ///
  /// ### Values
  ///
  /// Exchanges: the exchanges the node was registered.
  ///
  /// Vec<String>: the symbols the node was registered.
  Unregist(Box<Exchanges>, Vec<String>),
  Ping(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TradeObserverControlEvent {
  /// Triggered when the controller instructs the observer to add a symbol.
  SymbolAdd(Box<Exchanges>, String),
  /// Triggered when the controller instructs the observer to remove a symbol.
  SymbolDel(Box<Exchanges>, String),
}

impl From<SymbolEvent> for TradeObserverControlEvent {
  fn from(value: SymbolEvent) -> Self {
    return match value {
      SymbolEvent::Add(info) => {
        TradeObserverControlEvent::SymbolAdd(info.exchange, info.symbol)
      }
      SymbolEvent::Remove(info) => {
        TradeObserverControlEvent::SymbolDel(info.exchange, info.symbol)
      }
    };
  }
}
