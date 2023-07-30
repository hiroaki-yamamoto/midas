use ::mongodb::bson::DateTime;
use ::rug::Float;
use ::serde::{Deserialize, Serialize};

use ::errors::NotificationResult;

use super::account_update::AccountUpdate;
use super::balance_update::BalanceUpdate;
use super::execution_reports::ExecutionReport;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e", rename_all = "camelCase")]
pub enum UserStreamEvents<DT, FT> {
  OutboundAccountPosition(AccountUpdate<DT, FT>),
  BalanceUpdate(BalanceUpdate<DT, FT>),
  ExecutionReport(ExecutionReport<DT, FT>),
}

pub type RawUserStreamEvents = UserStreamEvents<i64, String>;
pub type CastedUserStreamEvents = UserStreamEvents<DateTime, Float>;

impl From<RawUserStreamEvents> for NotificationResult<CastedUserStreamEvents> {
  fn from(v: RawUserStreamEvents) -> Self {
    return Ok(match v {
      RawUserStreamEvents::OutboundAccountPosition(data) => {
        let data: NotificationResult<AccountUpdate<DateTime, Float>> =
          data.try_into();
        CastedUserStreamEvents::OutboundAccountPosition(data?)
      }
      RawUserStreamEvents::BalanceUpdate(data) => {
        let data: NotificationResult<BalanceUpdate<DateTime, Float>> =
          data.try_into();
        CastedUserStreamEvents::BalanceUpdate(data?)
      }
      RawUserStreamEvents::ExecutionReport(data) => {
        let data: NotificationResult<ExecutionReport<DateTime, Float>> =
          data.try_into();
        CastedUserStreamEvents::ExecutionReport(data?)
      }
    });
  }
}
