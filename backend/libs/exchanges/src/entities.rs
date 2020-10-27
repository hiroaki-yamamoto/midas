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

#[derive(Default)]
pub struct OrderOption {
  pub(crate) iceberg: bool,
  pub(crate) num_ladder: u8,
  pub(crate) price_ratio: f64, // Note: this value should be between -100 to 100 (i.e. in percentage.).
  pub(crate) qty_multiplyer: f64, // Note: Current qty = (previous qty) * qty_multiplyer
}

impl OrderOption {
  pub fn new() -> Self {
    return Self::default();
  }
  pub fn iceberg(&mut self, iceberg: bool) -> &mut Self {
    self.iceberg = iceberg;
    return self;
  }
  pub fn num_ladder(&mut self, num_ladder: u8) -> &mut Self {
    self.num_ladder = num_ladder;
    return self;
  }
  pub fn price_ratio(&mut self, price_ratio: f64) -> &mut Self {
    self.price_ratio = price_ratio;
    return self;
  }
  pub fn qty_multiplyer(&mut self, qty_multiplyer: f64) -> &mut Self {
    self.qty_multiplyer = qty_multiplyer;
    return self;
  }
}
