use ::serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct StreamResult {
  pub id: u64,
  pub result: Option<Vec<String>>
}
