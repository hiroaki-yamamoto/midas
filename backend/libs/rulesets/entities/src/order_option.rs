use ::num_traits::pow::pow;
use ::serde::{Deserialize, Serialize};
use ::types::stateful_setter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderOption {
  pub iceberg: bool,
  pub num_ladder: u8,
  pub price_ratio: f64,
  // Note: base_asset_amount[n] = base_asset_amount[n-1] * amount_multiplyer
  pub amount_multiplyer: f64,
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
  stateful_setter!(iceberg, bool);
  stateful_setter!(num_ladder, u8);
  stateful_setter!(price_ratio, f64);
  stateful_setter!(amount_multiplyer, f64);

  pub fn calc_order_price(&self, price: f64, num: usize) -> f64 {
    return price * pow(self.price_ratio, num);
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
