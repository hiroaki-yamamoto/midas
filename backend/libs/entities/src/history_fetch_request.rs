use ::chrono::{DateTime, Utc};
use ::mongodb::bson::DateTime as MongoDateTime;
use ::rpc::entities::Exchanges;
use ::rpc::historical::HistoryFetchRequest as RPCFetchReq;
use ::serde::{Deserialize, Serialize};
use ::std::time::Duration;
use ::std::time::SystemTime;
use ::types::stateful_setter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFetchRequest {
  pub exchange: Exchanges,
  pub symbol: String,
  pub start: Option<DateTime<Utc>>,
  pub end: Option<DateTime<Utc>>,
}

impl HistoryFetchRequest {
  pub fn new<T>(
    exchange: Exchanges,
    symbol: T,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
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
    let std_start: Option<SystemTime> = self.start.map(|start| start.into());
    if std_start.is_none() {
      return None;
    }
    let std_start = std_start.unwrap();
    return self
      .end
      .map(|end| {
        let end: SystemTime = end.into();
        end.duration_since(std_start).ok()
      })
      .flatten();
  }

  stateful_setter!(start, Option<DateTime<Utc>>);
  stateful_setter!(end, Option<DateTime<Utc>>);
}

impl From<RPCFetchReq> for HistoryFetchRequest {
  fn from(val: RPCFetchReq) -> Self {
    return Self {
      exchange: val.exchange(),
      symbol: val.symbol,
      start: val
        .start
        .map(|t| MongoDateTime::from_system_time(t.into()).to_chrono()),
      end: val
        .end
        .map(|t| MongoDateTime::from_system_time(t.into()).to_chrono()),
    };
  }
}

impl From<&RPCFetchReq> for HistoryFetchRequest {
  fn from(v: &RPCFetchReq) -> Self {
    return v.clone().into();
  }
}
