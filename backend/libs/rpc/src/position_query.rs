use super::pagination::Pagination;

#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionQuery {
  pub demo_mode: bool,
  pub pagination: Box<Pagination>,
}
