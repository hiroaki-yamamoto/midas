use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListenKey {
  pub listen_key: String,
}
