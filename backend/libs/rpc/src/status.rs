pub use ::http::StatusCode;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
  pub code: u32,
  pub message: String,
}
