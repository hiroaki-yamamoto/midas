use ::chrono::{DateTime, Utc};
use ::rpc::exchange::Exchange;
use ::rpc::history_fetch_request::HistoryFetchRequest as RPCFetchReq;
use ::rpc::timestamp::Timestamp;
use ::serde::{Deserialize, Serialize};
use ::std::time::Duration;
use ::std::time::SystemTime;
use ::types::stateful_setter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFetchRequest {
  pub exchange: Exchange,
  pub symbol: String,
  pub start: Option<DateTime<Utc>>,
  pub end: Option<DateTime<Utc>>,
}

impl HistoryFetchRequest {
  pub fn new(
    exchange: Exchange,
    symbol: &str,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
  ) -> Self {
    return Self {
      exchange,
      start,
      end,
      symbol: symbol.into(),
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
    let start: Option<DateTime<Utc>> = val
      .start
      .map(|dt| Timestamp::from(dt))
      .map(|dt| Option::<DateTime<Utc>>::from(dt))
      .flatten();
    let end: Option<DateTime<Utc>> = val
      .end
      .map(|dt| Timestamp::from(dt))
      .map(|dt| Option::<DateTime<Utc>>::from(dt))
      .flatten();
    return Self {
      exchange: val.exchange.into(),
      symbol: val.symbol,
      start,
      end,
    };
  }
}

impl From<&RPCFetchReq> for HistoryFetchRequest {
  fn from(v: &RPCFetchReq) -> Self {
    return v.clone().into();
  }
}
