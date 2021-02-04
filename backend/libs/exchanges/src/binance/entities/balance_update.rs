use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceUpdate<DT, FT> {
  #[serde(rename = "a")]
  pub asset: String,
  #[serde(rename = "d")]
  pub balance_delta: FT,
  #[serde(rename = "T")]
  pub clear_time: DT,
}
