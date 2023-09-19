use ::serde::{Deserialize, Serialize};

use ::entities::TradeObserverControlEvent;
use ::rpc::symbols::SymbolInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum SymbolEvent {
  Add(SymbolInfo),
  Remove(SymbolInfo),
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
