use ::std::convert::TryFrom;
use ::std::str::FromStr;

use ::mongodb::bson::{self, oid::ObjectId};
use ::rug::Float;
use ::serde::{Deserialize, Serialize};
use ::types::casting::cast_f_from_txt;

use ::errors::ParseError;
use ::rpc::bot::Bot as RPCBot;
use ::rpc::bot_mode::BotMode;
use ::rpc::bot_status::BotStatus;
use ::rpc::exchanges::Exchanges;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
  #[serde(rename = "_id", default)]
  pub id: ObjectId,
  pub name: String,
  pub mode: BotMode,
  pub status: BotStatus,
  pub base_currency: String,
  pub exchange: Box<Exchanges>,
  pub created_at: bson::DateTime,
  pub trading_amount: Float,
  pub cond_ts: String,
  pub cond_js: Option<String>,
}

impl Bot {
  pub fn new(
    id: Option<ObjectId>,
    name: String,
    base_currency: String,
    exchange: Exchanges,
    trading_amount: Float,
    cond: String,
  ) -> Self {
    let id: ObjectId = id.unwrap_or_else(ObjectId::new);
    return Self {
      id,
      name,
      base_currency,
      mode: BotMode::BackTest,
      status: BotStatus::Stopped,
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
      .flatten()
      .unwrap_or_else(ObjectId::new);
    let cond_ts = value.condition.ok_or(ParseError::new(
      Some("condition"),
      None,
      Some("Missing condition script"),
    ))?;
    return Ok(Self {
      id,
      name: value.name,
      base_currency: value.base_currency,
      mode: *value.mode,
      status: *value.status,
      exchange,
      created_at: bson::DateTime::now(),
      cond_ts,
      cond_js: None,
      trading_amount,
    });
  }
}

impl From<Bot> for RPCBot {
  fn from(value: Bot) -> Self {
    return Self {
      id: Some(value.id.to_hex()),
      name: value.name,
      mode: Box::new(value.mode),
      status: Box::new(value.status),
      base_currency: value.base_currency,
      exchange: value.exchange,
      created_at: Some(Box::new(value.created_at.to_chrono().into())),
      trading_amount: value.trading_amount.to_string(),
      condition: Some(value.cond_ts),
    };
  }
}
