use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIKey {
  pub exchange: String,
  pub label: String,
  pub pub_key: String,
  pub prv_key: String,
}

impl APIKey {
  fn new(
    exchange: String,
    label: String,
    pub_key: String,
    prv_key: String,
  ) -> Self {
    return Self {
      exchange,
      label,
      pub_key,
      prv_key,
    };
  }
}
