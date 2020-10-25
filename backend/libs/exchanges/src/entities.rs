use std::pin::Pin;

use ::futures::stream::Stream;
use ::rpc::entities::SymbolInfo;
use ::serde::{Deserialize, Serialize};

pub type ListSymbolStream =
  Pin<Box<dyn Stream<Item = SymbolInfo> + Send + 'static>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum KlineCtrl {
  Stop,
}
