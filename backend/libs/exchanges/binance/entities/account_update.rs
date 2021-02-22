use ::std::num::ParseFloatError;

use ::mongodb::bson::DateTime;
use ::serde::{Deserialize, Serialize};
use ::types::errors::{RawVecElemErrs, VecElementErr, VecElementErrs};

use crate::casting::cast_datetime_from_i64;

type ChangeAssetResult = Result<ChangedAsset<f64>, ParseFloatError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangedAsset<FloatType> {
  #[serde(rename = "a")]
  pub asset: String,
  #[serde(rename = "f")]
  pub free_amount: FloatType,
  #[serde(rename = "l")]
  pub locked_amount: FloatType,
}

impl From<ChangedAsset<String>> for ChangeAssetResult {
  fn from(v: ChangedAsset<String>) -> Self {
    return Ok(ChangedAsset::<f64> {
      asset: v.asset,
      free_amount: v.free_amount.parse()?,
      locked_amount: v.locked_amount.parse()?,
    });
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUpdate<DT, FT> {
  #[serde(rename = "u")]
  pub account_update_time: DT,
  #[serde(rename = "B")]
  pub updated_balances: Vec<ChangedAsset<FT>>,
}

impl From<AccountUpdate<i64, String>>
  for Result<AccountUpdate<DateTime, f64>, VecElementErrs<ParseFloatError>>
{
  fn from(v: AccountUpdate<i64, String>) -> Self {
    let (updated_balance, errors): (Vec<_>, Vec<_>) = v
      .updated_balances
      .into_iter()
      .map(|item| -> ChangeAssetResult { item.into() })
      .enumerate()
      .map(|(index, item)| {
        let mut err = None;
        if let Some(e) = item.clone().err() {
          err = Some(VecElementErr::new(index, e));
        }
        return (item.ok(), err);
      })
      .unzip();
    let errors: VecElementErrs<ParseFloatError> = errors
      .into_iter()
      .filter_map(|item| item)
      .collect::<RawVecElemErrs<ParseFloatError>>()
      .into();
    if !errors.errors.len() < 1 {
      return Err(errors);
    }
    let updated_balance = updated_balance
      .into_iter()
      .filter_map(|item| item)
      .collect();
    return Ok(AccountUpdate::<DateTime, f64> {
      account_update_time: cast_datetime_from_i64(v.account_update_time).into(),
      updated_balances: updated_balance,
    });
  }
}
