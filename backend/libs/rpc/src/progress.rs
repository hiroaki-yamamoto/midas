use super::exchanges::Exchanges;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
  pub cur: i64,
  pub exchange: Box<Exchanges>,
  pub size: i64,
  pub symbol: String,
}
