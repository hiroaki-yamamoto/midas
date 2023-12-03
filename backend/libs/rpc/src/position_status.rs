
#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PositionStatus {
  CLOSE,
  OPEN,
}
