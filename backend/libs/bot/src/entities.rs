use ::chrono::Utc;
use ::mongodb::bson;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
  pub id: bson::oid::ObjectId,
  pub name: String,
  pub created_at: bson::DateTime,
  pub trade_amount: f64,
  pub reinvest: bool,
  cond_ts: String,
  cond_js: Option<String>,
}

impl Bot {
  pub fn new(
    id: bson::oid::ObjectId,
    name: String,
    trade_amount: f64,
    reinvest: bool,
    cond: String,
  ) -> Self {
    return Self {
      id,
      name,
      trade_amount,
      reinvest,
      created_at: Utc::now().into(),
      cond_ts: cond,
      cond_js: None,
    };
  }
}
