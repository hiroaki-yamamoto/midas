use ::thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("Unknown Exchange: {}", exchange)]
pub struct UnknownExchangeError {
  exchange: String,
}

impl UnknownExchangeError {
  pub fn new(exchange: String) -> Self {
    return Self { exchange };
  }
}
