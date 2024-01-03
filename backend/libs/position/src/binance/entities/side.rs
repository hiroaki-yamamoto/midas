use ::std::ops::Not;
use ::std::str::FromStr;

use ::serde::{Deserialize, Serialize};

use ::errors::ParseError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
  Buy,
  Sell,
}

impl FromStr for Side {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "buy" => {
        return Ok(Self::Buy);
      }
      "sell" => {
        return Ok(Self::Sell);
      }
      _ => {
        return Err(ParseError::new(Some(s), None::<&str>, None::<&str>));
      }
    };
  }
}

impl Not for Side {
  type Output = Self;
  fn not(self) -> Self::Output {
    match self {
      Self::Buy => {
        return Self::Sell;
      }
      Self::Sell => {
        return Self::Buy;
      }
    };
  }
}
