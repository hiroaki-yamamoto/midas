use std::str::FromStr;

use ::errors::UnknownExchangeError;

use crate::exchanges::Exchanges;

impl Exchanges {
  pub fn as_str(&self) -> &str {
    match self {
      Exchanges::Binance => "Binance",
    }
  }
}

impl ToString for Exchanges {
  fn to_string(&self) -> String {
    return self.as_str().to_string();
  }
}

impl FromStr for Exchanges {
  type Err = UnknownExchangeError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Binance" => Ok(Exchanges::Binance),
      _ => Err(UnknownExchangeError::new(s.to_string())),
    }
  }
}
