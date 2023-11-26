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
