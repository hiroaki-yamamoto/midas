use std::pin::Pin;

use ::futures::stream::Stream;
use ::num::pow::pow;
use ::rpc::entities::SymbolInfo;
use ::serde::{Deserialize, Serialize};

pub type ListSymbolStream =
  Pin<Box<dyn Stream<Item = SymbolInfo> + Send + 'static>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum KlineCtrl {
  Stop,
}

pub struct OrderOption {
  pub(crate) iceberg: bool,
  pub(crate) num_ladder: u8,
  // Note: order_price[n] =
  //   order_price[n - 1] * (price_ratio)^n,
  //   where n in N & n > 0
  pub(crate) price_ratio: f64,
  // Note: base_asset_amount[n] = base_asset_amount[n-1] * amount_multiplyer
  pub(crate) amount_multiplyer: f64,
}

impl Default for OrderOption {
  fn default() -> Self {
    return Self {
      iceberg: false,
      num_ladder: 1,
      price_ratio: 0.0,
      amount_multiplyer: 1.0,
    };
  }
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
  pub fn amount_multiplyer(&mut self, amount_multiplyer: f64) -> &mut Self {
    self.amount_multiplyer = amount_multiplyer;
    return self;
  }
  pub fn calc_trading_amounts(&self, budget: f64) -> Vec<f64> {
    let init_amount =
      budget / pow(self.amount_multiplyer, self.num_ladder as usize);
    let mut ret = vec![];
    for i in 0..self.num_ladder {
      ret.push(init_amount * pow(self.amount_multiplyer, i as usize));
    }
    return ret;
  }
}
