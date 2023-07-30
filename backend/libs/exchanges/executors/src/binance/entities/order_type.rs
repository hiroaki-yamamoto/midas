use ::std::str::FromStr;

use ::serde::{Deserialize, Serialize};

use ::errors::ParseError;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
  Limit,
  Market,
  StopLoss,
  StopLossLimit,
  TakeProfit,
  TakeProfitLimit,
  LimitMaker,
}

impl FromStr for OrderType {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    return match s.to_uppercase().as_str() {
      "LIMIT" => Ok(Self::Limit),
      "MARKET" => Ok(Self::Market),
      "STOP_LOSS" => Ok(Self::StopLoss),
      "STOP_LOSS_LIMIT" => Ok(Self::StopLossLimit),
      "TAKE_PROFIT" => Ok(Self::TakeProfit),
      "TAKE_PROFIT_LIMIT" => Ok(Self::TakeProfitLimit),
      "LIMIT_MAKER" => Ok(Self::LimitMaker),
      _ => Err(ParseError::new(Some(s), None::<&str>, None::<&str>)),
    };
  }
}
