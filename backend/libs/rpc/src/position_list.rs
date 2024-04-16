use super::position::Position;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionList {
  pub positions: Vec<Box<Position>>,
}
