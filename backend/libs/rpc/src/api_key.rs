use super::exchanges::Exchanges;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
  pub exchange: Box<Exchanges>,
  pub id: Option<String>,
  pub label: String,
  pub prv_key: String,
  pub pub_key: String,
}
