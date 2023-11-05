use ::std::convert::TryFrom;
use ::std::str::FromStr;

use ::mongodb::bson;
use ::rug::Float;
use ::serde::{Deserialize, Serialize};
use ::types::casting::cast_f_from_txt;

use ::errors::ParseError;
use ::rpc::bot::{Bot as RPCBot, TimestampSchema as BotTS};
use ::rpc::exchange::Exchange;
use ::rpc::timestamp::Timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<bson::oid::ObjectId>,
  pub name: String,
  pub base_currency: String,
  pub exchange: Exchange,
  pub created_at: Option<bson::DateTime>,
  pub trading_amount: Float,
  pub cond_ts: String,
  pub cond_js: Option<String>,
}

impl Bot {
  pub fn new(
    id: Option<bson::oid::ObjectId>,
    name: String,
    base_currency: String,
    exchange: Exchange,
    trading_amount: Float,
    cond: String,
  ) -> Self {
    return Self {
      id,
      name,
      base_currency,
      exchange,
      trading_amount,
      created_at: None,
      cond_ts: cond,
      cond_js: None,
    };
  }
}

impl TryFrom<RPCBot> for Bot {
  type Error = ParseError;
  fn try_from(value: RPCBot) -> Result<Self, Self::Error> {
    let exchange = value.exchange.into();
    let trading_amount =
      cast_f_from_txt("trading_amount", value.trading_amount.as_str())?;
    return Ok(Self {
      id: value
        .id
        .map(|id| bson::oid::ObjectId::from_str(&id).ok())
        .flatten(),
      name: value.name,
      base_currency: value.base_currency,
      exchange,
      created_at: None,
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
      exchange: value.exchange.into(),
      created_at: value.created_at.map(|time| {
        let ts: Timestamp = time.into();
        let ts: BotTS = ts.into();
        return ts;
      }),
      trading_amount: value.trading_amount.to_string(),
      condition: value.cond_ts,
    };
  }
}
