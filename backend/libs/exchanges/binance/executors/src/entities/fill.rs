use std::convert::TryFrom;

use ::entities::OrderInner;
use ::errors::ParseError;
use ::serde::{Deserialize, Serialize};

use super::side::Side;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill<FT> {
  pub price: FT,
  pub qty: FT,
  pub commission: FT,
  pub commissionAsset: String,
}

impl Fill<f64> {
  pub fn as_order_inner(&self, side: Side) -> OrderInner {
    let qty = match side {
      Side::Sell => ((self.price * self.qty) - self.commission) / self.price,
      Side::Buy => self.qty - self.commission,
    };
    return OrderInner {
      price: self.price,
      qty,
    };
  }
}

impl TryFrom<Fill<String>> for Fill<f64> {
  type Error = ParseError;
  fn try_from(v: Fill<String>) -> Result<Fill<f64>, Self::Error> {
    return Ok(Fill::<f64> {
      price: v
        .price
        .parse()
        .map_err(|e| ParseError::new(format!("price: {}", v.price)))?,
      qty: v
        .qty
        .parse()
        .map_err(|e| ParseError::new(format!("qty: {}", v.qty)))?,
      commission: v
        .commission
        .parse()
        .map_err(|e| ParseError::new(format!("commission: {}", v.qty)))?,
      commissionAsset: v.commissionAsset,
    });
  }
}
