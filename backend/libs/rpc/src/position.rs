use super::bot_mode::BotMode;
use super::position_status::PositionStatus;
use super::timestamp::Timestamp;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
  pub amount: String,
  pub bot_id: String,
  pub entry_at: Box<Timestamp>,
  pub entry_price: String,
  pub exit_at: Option<Box<Timestamp>>,
  pub exit_price: Option<String>,
  pub id: String,
  pub mode: Box<BotMode>,
  pub status: Box<PositionStatus>,
  pub symbol: String,
}
