use std::pin::Pin;

use ::futures::stream::Stream;
use ::serde::{Deserialize, Serialize};

use ::rpc::entities::SymbolInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum KlineCtrl {
  Stop,
}

pub type ListSymbolStream =
  Pin<Box<dyn Stream<Item = SymbolInfo> + Send + 'static>>;
