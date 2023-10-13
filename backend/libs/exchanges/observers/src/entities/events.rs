use ::serde::{Deserialize, Serialize};

use ::rpc::entities::Exchanges;
use ::symbols::entities::SymbolEvent;

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

impl From<SymbolEvent> for TradeObserverControlEvent {
  fn from(value: SymbolEvent) -> Self {
    return match value {
      SymbolEvent::Add(info) => {
        TradeObserverControlEvent::SymbolAdd(info.exchange(), info.symbol)
      }
      SymbolEvent::Remove(info) => {
        TradeObserverControlEvent::SymbolDel(info.exchange(), info.symbol)
      }
    };
  }
}
