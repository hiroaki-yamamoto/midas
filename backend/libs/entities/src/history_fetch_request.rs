use ::mongodb::bson::DateTime;
use ::rpc::entities::Exchanges;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFetchRequest {
  pub exchange: Exchanges,
  pub symbol: String,
  pub start: DateTime,
  pub end: Option<DateTime>,
}

impl HistoryFetchRequest {
  pub fn new<T>(
    exchange: Exchanges,
    symbol: T,
    start: DateTime,
    end: Option<DateTime>,
  ) -> Self
  where
    T: AsRef<str>,
  {
    return Self {
      exchange,
      start,
      end,
      symbol: symbol.as_ref().to_string(),
    };
  }
}
