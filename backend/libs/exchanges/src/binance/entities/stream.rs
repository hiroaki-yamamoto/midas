use ::serde::{Deserialize, Serialize};

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
pub enum UserStreamEvents {
  OutboundAccountPosition(AccountUpdate<u64, f64>),
  BalanceUpdate(BalanceUpdate<u64, f64>),
  ExecutionReport(ExecutionReport<u64, f64>),
}
