use ::std::num::ParseFloatError;

use ::mongodb::bson::DateTime;
use ::serde::{Deserialize, Serialize};

use ::types::errors::VecElementErrs;
use ::types::GenericResult;

use super::account_update::AccountUpdate;
use super::balance_update::BalanceUpdate;
use super::execution_reports::ExecutionReport;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeRequestInner {
  pub id: u64,
  pub params: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "method")]
pub enum SubscribeRequest {
  #[serde(rename = "SUBSCRIBE")]
  Subscribe(SubscribeRequestInner),
  #[serde(rename = "UNSUBSCRIBE")]
  Unsubscribe(SubscribeRequestInner),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e", rename_all = "camelCase")]
pub enum UserStreamEvents<DT, FT> {
  OutboundAccountPosition(AccountUpdate<DT, FT>),
  BalanceUpdate(BalanceUpdate<DT, FT>),
  ExecutionReport(ExecutionReport<DT, FT>),
}

pub type RawUserStreamEvents = UserStreamEvents<i64, String>;
pub type CastedUserStreamEvents = UserStreamEvents<DateTime, f64>;

impl From<RawUserStreamEvents> for GenericResult<CastedUserStreamEvents> {
  fn from(v: RawUserStreamEvents) -> Self {
    return Ok(match v {
      RawUserStreamEvents::OutboundAccountPosition(data) => {
        let data: Result<
          AccountUpdate<DateTime, f64>,
          VecElementErrs<ParseFloatError>,
        > = data.into();
        CastedUserStreamEvents::OutboundAccountPosition(data?)
      }
      RawUserStreamEvents::BalanceUpdate(data) => {
        let data: Result<BalanceUpdate<DateTime, f64>, ParseFloatError> =
          data.into();
        CastedUserStreamEvents::BalanceUpdate(data?)
      }
      RawUserStreamEvents::ExecutionReport(data) => {
        let data: Result<ExecutionReport<DateTime, f64>, ParseFloatError> =
          data.into();
        CastedUserStreamEvents::ExecutionReport(data?)
      }
    });
  }
}
