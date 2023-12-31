use ::mongodb::bson::{oid::ObjectId, DateTime};
use ::serde::{Deserialize, Serialize};

use ::types::chrono::Utc;
use ::types::stateful_setter;

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub bot_id: ObjectId,
  pub entry_at: DateTime,
  pub entry_ids: Vec<ObjectId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exit_at: Option<DateTime>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub exit_ids: Vec<ObjectId>,
  pub symbol: String,
}

impl Position {
  pub fn new(bot_id: ObjectId, symbol: String) -> Self {
    return Self {
      id: ObjectId::new(),
      bot_id,
      symbol,
      entry_at: Utc::now().into(),
      entry_ids: vec![],
      exit_at: None,
      exit_ids: vec![],
    };
  }

  stateful_setter!(exit_at, Option<DateTime>);
}
