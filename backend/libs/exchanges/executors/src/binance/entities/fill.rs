use std::convert::TryFrom;

use ::rug::Float;
use ::serde::{Deserialize, Serialize};

use ::entities::OrderInner;
use ::errors::ParseError;

use super::side::Side;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill<FT> {
  pub price: FT,
  pub qty: FT,
  pub commission: FT,
  pub commission_asset: String,
}

impl Fill<Float> {
  pub fn as_order_inner(&self, side: Side) -> OrderInner {
    let qty = match side {
      Side::Sell => {
        ((self.price.clone() * &self.qty) - &self.commission) / &self.price
      }
      Side::Buy => self.qty.clone() - &self.commission,
    };
    return OrderInner {
      price: self.price.clone(),
      qty,
    };
  }
}

impl TryFrom<Fill<String>> for Fill<Float> {
  type Error = ParseError;
  fn try_from(v: Fill<String>) -> Result<Fill<Float>, Self::Error> {
    let price = Float::parse(&v.price)
      .map_err(Self::Error::raise_parse_err("price", v.price))?;
    let qty = Float::parse(&v.qty)
      .map_err(Self::Error::raise_parse_err("price", v.qty))?;
    let commission = Float::parse(&v.commission)
      .map_err(Self::Error::raise_parse_err("commission", v.commission))?;

    let price = Float::with_val(32, price);
    let qty = Float::with_val(32, qty);
    let commission = Float::with_val(32, commission);
    return Ok(Fill::<Float> {
      price,
      qty,
      commission,
      commission_asset: v.commission_asset,
    });
  }
}
