use ::chrono::Utc;
use ::mongodb::bson;
use ::rpc::entities::Exchanges;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
  pub id: bson::oid::ObjectId,
  pub name: String,
  pub exchange: Exchanges,
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
    exchange: Exchanges,
    trade_amount: f64,
    reinvest: bool,
    cond: String,
  ) -> Self {
    return Self {
      id,
      name,
      exchange,
      trade_amount,
      reinvest,
      created_at: Utc::now().into(),
      cond_ts: cond,
      cond_js: None,
    };
  }
}
