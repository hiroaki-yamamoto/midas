use super::bot_mode::BotMode;
use super::bot_status::BotStatus;
use super::exchanges::Exchanges;
use super::timestamp::Timestamp;

#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotResponse {
  pub base_currency: String,
  pub condition: String,
  pub created_at: Box<Timestamp>,
  pub exchange: Box<Exchanges>,
  pub id: String,
  pub mode: Box<BotMode>,
  pub name: String,
  pub status: Box<BotStatus>,
  pub trading_amount: String,
}
