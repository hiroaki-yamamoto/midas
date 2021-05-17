use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetIndicator {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompareOp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionItem {
  pub op_a: TargetIndicator,
  pub op_b: TargetIndicator,
  pub op: CompareOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
  Or(Vec<Condition>),
  And(Vec<Condition>),
  Single(ConditionItem),
}
