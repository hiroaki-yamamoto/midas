use std::num::ParseFloatError;

use ::mongodb::bson::DateTime;
use ::serde::{Deserialize, Serialize};

use ::types::casting::cast_datetime_from_i64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceUpdate<DT, FT> {
  #[serde(rename = "a")]
  pub asset: String,
  #[serde(rename = "d")]
  pub balance_delta: FT,
  #[serde(rename = "T")]
  pub clear_time: DT,
}

impl From<BalanceUpdate<i64, String>>
  for Result<BalanceUpdate<DateTime, f64>, ParseFloatError>
{
  fn from(v: BalanceUpdate<i64, String>) -> Self {
    return Ok(BalanceUpdate::<DateTime, f64> {
      asset: v.asset,
      balance_delta: v.balance_delta.parse()?,
      clear_time: cast_datetime_from_i64(v.clear_time).into(),
    });
  }
}
