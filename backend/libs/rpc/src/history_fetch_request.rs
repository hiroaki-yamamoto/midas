use super::exchanges::Exchanges;
use super::timestamp::Timestamp;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryFetchRequest {
  pub end: Box<Timestamp>,
  pub exchange: Box<Exchanges>,
  pub start: Box<Timestamp>,
  pub symbol: String,
}
