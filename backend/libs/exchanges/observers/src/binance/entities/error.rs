use ::serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Error {
  pub code: u64,
  pub msg: String,
  pub id: Option<String>,
}
