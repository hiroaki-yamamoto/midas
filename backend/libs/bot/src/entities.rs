use std::convert::TryFrom;
use std::str::FromStr;

use ::chrono::Utc;
use ::errors::ParseError;
use ::mongodb::bson;
use ::rpc::bot::Bot as RPCBot;
use ::rpc::entities::Exchanges;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
  pub id: Option<bson::oid::ObjectId>,
  pub name: String,
  pub exchange: Exchanges,
  pub created_at: Option<bson::DateTime>,
  pub trade_amount: f64,
  pub reinvest: bool,
  cond_ts: String,
  cond_js: Option<String>,
}

impl Bot {
  pub fn new(
    id: Option<bson::oid::ObjectId>,
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
      created_at: None,
      cond_ts: cond,
      cond_js: None,
    };
  }
}

impl TryFrom<RPCBot> for Bot {
  type Error = ParseError;
  fn try_from(value: RPCBot) -> Result<Self, Self::Error> {
    return Ok(Self {
      id: bson::oid::ObjectId::from_str(value.id.as_str()).ok(),
      name: value.name,
      exchange: Exchanges::try_from(value.exchange)?,
      trade_amount: value.trade_amount,
      created_at: None,
      reinvest: value.reinvest,
      cond_ts: value.condition,
      cond_js: None,
    });
  }
}
