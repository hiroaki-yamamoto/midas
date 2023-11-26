use ::rpc::exchanges::Exchanges;
use ::rpc::history_fetch_request::HistoryFetchRequest as RPCFetchReq;
use ::serde::{Deserialize, Serialize};
use ::std::time::Duration;
use ::std::time::SystemTime;
use ::types::{stateful_setter, DateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFetchRequest {
  pub exchange: Box<Exchanges>,
  pub symbol: String,
  pub start: Option<DateTime>,
  pub end: Option<DateTime>,
}

impl HistoryFetchRequest {
  pub fn new(
    exchange: Exchanges,
    symbol: &str,
    start: Option<DateTime>,
    end: Option<DateTime>,
  ) -> Self {
    return Self {
      exchange: Box::new(exchange),
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

  stateful_setter!(start, Option<DateTime>);
  stateful_setter!(end, Option<DateTime>);
}

impl From<RPCFetchReq> for HistoryFetchRequest {
  fn from(val: RPCFetchReq) -> Self {
    let start: Option<DateTime> = (*val.start).try_into().ok();
    let end: Option<DateTime> = (*val.end).try_into().ok();
    return Self {
      exchange: val.exchange,
      symbol: val.symbol,
      start: start.into(),
      end: end.into(),
    };
  }
}

impl From<&RPCFetchReq> for HistoryFetchRequest {
  fn from(v: &RPCFetchReq) -> Self {
    return v.clone().into();
  }
}
