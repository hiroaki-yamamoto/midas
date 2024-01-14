use super::bot_mode::BotMode;
use super::bot_status::BotStatus;
use super::exchanges::Exchanges;
use super::timestamp::Timestamp;

#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
  pub base_currency: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub condition: Option<String>,
  pub created_at: Option<Box<Timestamp>>,
  pub exchange: Box<Exchanges>,
  pub id: Option<String>,
  pub mode: Box<BotMode>,
  pub name: String,
  pub status: Box<BotStatus>,
  pub trading_amount: String,
}
