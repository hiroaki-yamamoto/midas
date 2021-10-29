use ::mongodb::bson::DateTime as MongoDateTime;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
  pub symbol: String,
  pub num_symbols: i64,
  pub entire_data_len: u64,
  pub start_time: MongoDateTime,
  pub end_time: Option<MongoDateTime>,
}

impl AsRef<Param> for Param {
  fn as_ref(&self) -> &Self {
    return self;
  }
}
