use super::exchanges::Exchanges;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
  pub base_currency: String,
  pub condition: String,
  pub created_at: String,
  pub exchange: Box<Exchanges>,
  pub id: String,
  pub name: String,
  pub trading_amount: String,
}
