use ::rug::ops::Pow;
use ::rug::Float;
use ::serde::{Deserialize, Serialize};
use ::types::stateful_setter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderOption {
  pub iceberg: bool,
  pub num_ladder: u8,
  pub price_ratio: Float,
  // Note: base_asset_amount[n] = base_asset_amount[n-1] * amount_multiplyer
  pub amount_multiplyer: Float,
}

impl Default for OrderOption {
  fn default() -> Self {
    return Self {
      iceberg: false,
      num_ladder: 1,
      price_ratio: Float::with_val(32, 0.0),
      amount_multiplyer: Float::with_val(32, 1.0),
    };
  }
}

impl OrderOption {
  pub fn new() -> Self {
    return Self::default();
  }
  stateful_setter!(iceberg, bool);
  stateful_setter!(num_ladder, u8);
  stateful_setter!(price_ratio, Float);
  stateful_setter!(amount_multiplyer, Float);

  pub fn calc_order_price(&self, price: Float, num: usize) -> Float {
    return price * self.price_ratio.clone().pow(num);
  }

  pub fn calc_trading_amounts(&self, budget: Float) -> Vec<Float> {
    let init_amount =
      budget / self.amount_multiplyer.clone().pow(self.num_ladder as usize);
    let mut ret = vec![];
    for i in 0..self.num_ladder {
      ret.push(
        init_amount.clone() * self.amount_multiplyer.clone().pow(i as usize),
      );
    }
    return ret;
  }
}
