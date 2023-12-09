use ::std::convert::TryFrom;
use ::std::str::FromStr;

use ::mongodb::bson;
use ::rug::Float;
use ::serde::{Deserialize, Serialize};
use ::types::casting::cast_f_from_txt;

use ::errors::ParseError;
use ::rpc::bot::Bot as RPCBot;
use ::rpc::exchanges::Exchanges;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<bson::oid::ObjectId>,
  pub name: String,
  pub base_currency: String,
  pub exchange: Box<Exchanges>,
  pub created_at: bson::DateTime,
  pub trading_amount: Float,
  pub cond_ts: String,
  pub cond_js: Option<String>,
}

impl Bot {
  pub fn new(
    id: Option<bson::oid::ObjectId>,
    name: String,
    base_currency: String,
    exchange: Exchanges,
    trading_amount: Float,
    cond: String,
  ) -> Self {
    return Self {
      id,
      name,
      base_currency,
      exchange: Box::new(exchange),
      trading_amount,
      created_at: bson::DateTime::now(),
      cond_ts: cond,
      cond_js: None,
    };
  }
}

impl TryFrom<RPCBot> for Bot {
  type Error = ParseError;
  fn try_from(value: RPCBot) -> Result<Self, Self::Error> {
    let exchange = value.exchange;
    let trading_amount = value.trading_amount;
    let trading_amount =
      cast_f_from_txt("trading_amount", trading_amount.as_str())?;
    let id = value
      .id
      .map(|id| bson::oid::ObjectId::from_str(&id).ok())
      .flatten();
    return Ok(Self {
      id,
      name: value.name,
      base_currency: value.base_currency,
      exchange,
      created_at: bson::DateTime::now(),
      cond_ts: value.condition,
      cond_js: None,
      trading_amount,
    });
  }
}

impl From<Bot> for RPCBot {
  fn from(value: Bot) -> Self {
    return Self {
      id: value.id.map(|id| id.to_hex()),
      name: value.name,
      base_currency: value.base_currency,
      exchange: value.exchange,
      created_at: Box::new(value.created_at.to_chrono().into()),
      trading_amount: value.trading_amount.to_string(),
      condition: value.cond_ts,
    };
  }
}
