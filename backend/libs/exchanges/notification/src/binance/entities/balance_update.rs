use ::errors::{NotificationError, NotificationResult};
use ::mongodb::bson::DateTime;
use ::rug::Float;
use ::serde::{Deserialize, Serialize};
use ::std::convert::TryFrom;

use ::types::casting::{cast_datetime_from_i64, cast_f_from_txt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceUpdate<DT, FT> {
  #[serde(rename = "a")]
  pub asset: String,
  #[serde(rename = "d")]
  pub balance_delta: FT,
  #[serde(rename = "T")]
  pub clear_time: DT,
}

impl TryFrom<BalanceUpdate<i64, String>> for BalanceUpdate<DateTime, Float> {
  type Error = NotificationError;
  fn try_from(v: BalanceUpdate<i64, String>) -> NotificationResult<Self> {
    let balance_delta = cast_f_from_txt("balance_delta", &v.balance_delta)?;
    return Ok(BalanceUpdate::<DateTime, Float> {
      asset: v.asset,
      balance_delta,
      clear_time: cast_datetime_from_i64(v.clear_time).into(),
    });
  }
}
