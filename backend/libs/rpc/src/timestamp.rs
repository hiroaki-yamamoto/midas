
#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
  pub nanos: u32,
  pub secs: i64,
}
