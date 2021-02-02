use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangedAsset<FloatType> {
  #[serde(rename = "a")]
  pub asset: String,
  #[serde(rename = "f")]
  pub free_amount: FloatType,
  #[serde(rename = "l")]
  pub locked_amount: FloatType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUpdate<DT, FT> {
  #[serde(rename = "E")]
  pub event_time: DT,
  #[serde(rename = "u")]
  pub account_update_time: DT,
  #[serde(rename = "B")]
  pub updated_balance: Vec<ChangedAsset<FT>>,
}
