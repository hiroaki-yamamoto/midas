use ::mongodb::bson::DateTime;
use ::rpc::entities::Exchanges;
use ::serde::{Deserialize, Serialize};
use ::std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFetchRequest {
  pub exchange: Exchanges,
  pub symbol: String,
  pub start: Option<DateTime>,
  pub end: Option<DateTime>,
}

impl HistoryFetchRequest {
  pub fn new<T>(
    exchange: Exchanges,
    symbol: T,
    start: Option<DateTime>,
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

  pub fn duration(&self) -> Option<Duration> {
    let std_start = self.start.map(|start| start.to_system_time());
    if std_start.is_none() {
      return None;
    }
    let std_start = std_start.unwrap();
    return self
      .end
      .map(|end| end.to_system_time().duration_since(std_start).ok())
      .flatten();
  }
}
