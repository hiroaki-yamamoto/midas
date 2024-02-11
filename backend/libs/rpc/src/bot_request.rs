use super::exchanges::Exchanges;

#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotRequest {
  pub base_currency: String,
  pub condition: String,
  pub exchange: Box<Exchanges>,
  pub id: Option<String>,
  pub name: String,
  pub trading_amount: String,
}
