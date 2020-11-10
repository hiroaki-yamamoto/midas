use super::entities::Exchanges;

impl Exchanges {
  pub fn as_string(&self) -> String {
    return match self {
      Exchanges::Binance => String::from("binance"),
    };
  }
}

impl ::std::str::FromStr for Exchanges {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let ret: Self = match s {
      "binance" => Exchanges::Binance,
      _ => return Err(format!("Failed to parse the exchange argument: {}", s)),
    };
    return Ok(ret);
  }
}

impl From<Exchanges> for String {
  fn from(exchange: Exchanges) -> Self {
    return exchange.as_string();
  }
}
