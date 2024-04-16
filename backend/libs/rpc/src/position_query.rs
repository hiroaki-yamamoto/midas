use super::pagination::Pagination;

#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionQuery {
  #[cfg(debug_assertions)]
  pub demo_mode: bool,
  pub pagination: Box<Pagination>,
}
