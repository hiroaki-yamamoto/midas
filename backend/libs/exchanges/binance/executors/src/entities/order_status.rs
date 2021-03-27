use std::str::FromStr;

use ::errors::ParseError;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
  New,
  PartiallyFilled,
  Filled,
  Canceled,
  PendingCancel,
  Rejected,
  Expired,
}

impl FromStr for OrderStatus {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_uppercase().as_str() {
      "NEW" => {
        return Ok(Self::New);
      }
      "PARTIALLY_FILLED" => {
        return Ok(Self::PartiallyFilled);
      }
      "FILLED" => {
        return Ok(Self::Filled);
      }
      "CANCELED" => {
        return Ok(Self::Canceled);
      }
      "PENDING_CANCEL" => {
        return Ok(Self::PendingCancel);
      }
      "REJECTED" => {
        return Ok(Self::Rejected);
      }
      "EXPIRED" => {
        return Ok(Self::Expired);
      }
      _ => {
        return Err(ParseError::new::<&str, String>(Some(s), None));
      }
    }
  }
}
