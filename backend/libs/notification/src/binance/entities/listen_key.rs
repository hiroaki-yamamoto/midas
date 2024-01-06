use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListenKey {
  pub listen_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenKeyPair {
  pub listen_key: String,
  pub pub_key: String,
}

impl ListenKeyPair {
  pub fn new(listen_key: String, pub_key: String) -> Self {
    return Self {
      listen_key,
      pub_key,
    };
  }
}

impl From<ListenKeyPair> for ListenKey {
  fn from(listen_key_pair: ListenKeyPair) -> Self {
    return Self {
      listen_key: listen_key_pair.listen_key,
    };
  }
}
