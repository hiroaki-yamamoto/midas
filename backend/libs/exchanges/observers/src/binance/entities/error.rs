use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Error {
  pub code: u64,
  pub msg: String,
  pub id: Option<String>,
}
