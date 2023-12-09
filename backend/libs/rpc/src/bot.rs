use super::exchanges::Exchanges;
use super::timestamp::Timestamp;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
  pub base_currency: String,
  pub condition: String,
  pub created_at: Box<Timestamp>,
  pub exchange: Box<Exchanges>,
  pub id: Option<String>,
  pub name: String,
  pub trading_amount: String,
}
