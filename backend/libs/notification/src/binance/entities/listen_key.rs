use ::keychain::APIKey;
use ::serde::{Deserialize, Serialize};
use ::std::sync::Arc;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenKey {
  pub listen_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenKeyPair {
  pub listen_key: String,
  pub api_key: Arc<APIKey>,
}

impl ListenKeyPair {
  pub fn new(listen_key: String, api_key: Arc<APIKey>) -> Self {
    return Self {
      listen_key,
      api_key,
    };
  }
}

impl From<&ListenKeyPair> for ListenKey {
  fn from(listen_key_pair: &ListenKeyPair) -> Self {
    return Self {
      listen_key: listen_key_pair.listen_key.clone(),
    };
  }
}
