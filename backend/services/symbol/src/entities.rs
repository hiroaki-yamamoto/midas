use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct BaseCurrencies {
  currencies: Vec<String>,
}

impl BaseCurrencies {
  pub fn new(base: Vec<String>) -> Self {
    return Self { currencies: base };
  }
}
