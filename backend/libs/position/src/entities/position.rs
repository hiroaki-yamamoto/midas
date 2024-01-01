use ::mongodb::bson::{oid::ObjectId, DateTime};
use ::serde::{Deserialize, Serialize};

use ::types::chrono::Utc;
use ::types::stateful_setter;

use ::rpc::bot_mode::BotMode;

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub mode: BotMode,
  pub bot_id: ObjectId,
  pub symbol: String,
  pub entry_at: DateTime,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exit_at: Option<DateTime>,
  pub entry_gid: ObjectId,
  pub exit_gid: ObjectId,
}

impl Position {
  pub fn new(bot_id: ObjectId, mode: BotMode, symbol: &str) -> Self {
    return Self {
      id: ObjectId::new(),
      mode,
      bot_id,
      symbol: symbol.to_string(),
      entry_at: Utc::now().into(),
      entry_gid: ObjectId::new(),
      exit_at: None,
      exit_gid: ObjectId::new(),
    };
  }

  stateful_setter!(exit_at, Option<DateTime>);
}
