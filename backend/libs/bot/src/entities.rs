use ::std::convert::TryFrom;
use ::std::str::FromStr;

use ::mongodb::bson;
use ::serde::{Deserialize, Serialize};

use ::errors::ParseError;
use ::rpc::bot::Bot as RPCBot;
use ::rpc::entities::Exchanges;
use ::types::DateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
  pub id: Option<bson::oid::ObjectId>,
  pub name: String,
  pub exchange: Exchanges,
  pub created_at: Option<bson::DateTime>,
  pub trade_amount: f64,
  pub reinvest: bool,
  pub cond_ts: String,
  pub cond_js: Option<String>,
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

impl From<Bot> for RPCBot {
  fn from(value: Bot) -> Self {
    return Self {
      id: value.id.map(|id| id.to_hex()).unwrap_or("".to_string()),
      name: value.name,
      exchange: value.exchange as i32,
      created_at: value.created_at.map(|time| {
        let time: DateTime = time.into();
        time.into()
      }),
      trade_amount: value.trade_amount,
      reinvest: value.reinvest,
      condition: value.cond_ts,
    };
  }
}
