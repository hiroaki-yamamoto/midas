use ::std::convert::From;
use ::std::time::{SystemTime, UNIX_EPOCH};

use ::entities::HistoryFetchRequest;
use ::serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
  pub symbol: String,
  pub interval: String,
  pub start_time: Option<String>,
  pub end_time: Option<String>,
  pub limit: String,
}

impl From<HistoryFetchRequest> for Query {
  fn from(value: HistoryFetchRequest) -> Self {
    return Self::from(&value);
  }
}

impl From<&HistoryFetchRequest> for Query {
  fn from(value: &HistoryFetchRequest) -> Self {
    let std_start: Option<SystemTime> = value.start.map(|d| d.into());
    let std_end: Option<SystemTime> = value.end.map(|d| d.into());

    return Self {
      symbol: value.symbol.clone(),
      start_time: std_start
        .map(|start| start.duration_since(UNIX_EPOCH).ok())
        .flatten()
        .map(|start| start.as_millis().to_string()),
      end_time: std_end
        .map(|std_end| std_end.duration_since(UNIX_EPOCH).ok())
        .flatten()
        .map(|std_end| std_end.as_millis().to_string()),
      interval: "1m".into(),
      limit: value
        .duration()
        .map(|dur| (dur.as_secs() / 60))
        .unwrap_or(1)
        .to_string(),
    };
  }
}
