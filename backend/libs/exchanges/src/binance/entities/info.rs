use ::serde::{Deserialize, Serialize};
use ::serde_json::Value;

use super::Symbol;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
  pub timezone: String,
  pub exchange_filters: Vec<Value>,
  pub symbols: Vec<Symbol>,
}
