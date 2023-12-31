use ::mongodb::bson::{oid::ObjectId, DateTime};
use ::serde::{Deserialize, Serialize};
use ::types::stateful_setter;

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
  #[serde(rename = "_id")]
  id: ObjectId,
  bot_id: ObjectId,
  entry_at: DateTime,
  entry_ids: Vec<ObjectId>,
  exit_at: Option<DateTime>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  exit_ids: Vec<ObjectId>,
  symbol: String,
}

impl Position {
  pub fn new(bot_id: ObjectId, symbol: String, entry_at: DateTime) -> Self {
    return Self {
      id: ObjectId::new(),
      bot_id,
      symbol,
      entry_at,
      entry_ids: vec![],
      exit_at: None,
      exit_ids: vec![],
    };
  }

  stateful_setter!(exit_at, Option<DateTime>);
}
