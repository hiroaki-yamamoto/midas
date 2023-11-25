use super::exchanges::Exchanges;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCheckRequest {
  pub exchange: Box<Exchanges>,
  pub symbol: String,
}
