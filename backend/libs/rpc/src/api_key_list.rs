use super::api_key::ApiKey;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyList {
  pub keys: Vec<Box<ApiKey>>,
}
