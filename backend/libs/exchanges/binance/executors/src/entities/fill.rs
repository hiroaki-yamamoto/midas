use std::convert::TryFrom;

use ::errors::ParseError;
use ::serde::{Deserialize, Serialize};

use ::entities::OrderInner;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill<FT> {
  pub price: FT,
  pub qty: FT,
  pub commission: FT,
  pub commissionAsset: String,
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
