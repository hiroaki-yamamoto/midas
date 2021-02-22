use ::std::str::FromStr;

use ::serde::{Deserialize, Serialize};

use crate::errors::ParseError;

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
        return Err(ParseError::new(s.to_string()));
      }
    };
  }
}
