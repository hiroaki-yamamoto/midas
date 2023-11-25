use super::exchanges::Exchanges;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
  pub cur: String,
  pub exchange: Box<Exchanges>,
  pub size: String,
  pub symbol: String,
}
