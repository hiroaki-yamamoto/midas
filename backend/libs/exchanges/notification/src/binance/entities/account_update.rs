use ::std::convert::TryFrom;

use ::mongodb::bson::DateTime;
use ::rug::Float;
use ::serde::{Deserialize, Serialize};

use ::errors::{
  NotificationError, NotificationResult, ParseError, RawVecElemErrs,
  VecElementErr, VecElementErrs,
};
use ::types::casting::{cast_datetime_from_i64, cast_f_from_txt};

type ChangeAssetResult<F> = Result<ChangedAsset<F>, ParseError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangedAsset<FloatType> {
  #[serde(rename = "a")]
  pub asset: String,
  #[serde(rename = "f")]
  pub free_amount: FloatType,
  #[serde(rename = "l")]
  pub locked_amount: FloatType,
}

impl TryFrom<ChangedAsset<String>> for ChangedAsset<Float> {
  type Error = ParseError;
  fn try_from(v: ChangedAsset<String>) -> ChangeAssetResult<Float> {
    let free_amount = cast_f_from_txt("free_amount", &v.free_amount)?;
    let locked_amount = cast_f_from_txt("locked_amount", &v.locked_amount)?;
    return Ok(ChangedAsset::<Float> {
      asset: v.asset,
      free_amount,
      locked_amount,
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

impl TryFrom<AccountUpdate<i64, String>> for AccountUpdate<DateTime, Float> {
  type Error = NotificationError;
  fn try_from(v: AccountUpdate<i64, String>) -> NotificationResult<Self> {
    let (updated_balance, errors): (Vec<_>, Vec<_>) = v
      .updated_balances
      .into_iter()
      .map(|item| -> ChangeAssetResult<Float> { item.try_into() })
      .enumerate()
      .map(|(index, item)| {
        let mut err = None;
        if let Some(e) = item.clone().err() {
          err = Some(VecElementErr::new(index, e));
        }
        return (item.ok(), err);
      })
      .unzip();
    let errors: VecElementErrs<ParseError> = errors
      .into_iter()
      .filter_map(|item| item)
      .collect::<RawVecElemErrs<ParseError>>()
      .into();
    if !errors.errors.len() < 1 {
      return Err(errors.into());
    }
    let updated_balance = updated_balance
      .into_iter()
      .filter_map(|item| item)
      .collect();
    return Ok(AccountUpdate::<DateTime, Float> {
      account_update_time: cast_datetime_from_i64(v.account_update_time).into(),
      updated_balances: updated_balance,
    });
  }
}
