use super::exchanges::Exchanges;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryFetchRequest {
  pub end: String,
  pub exchange: Box<Exchanges>,
  pub start: String,
  pub symbol: String,
}
