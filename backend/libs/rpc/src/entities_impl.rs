use super::entities::Exchanges;

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
