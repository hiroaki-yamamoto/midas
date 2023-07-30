use ::std::convert::TryFrom;
use ::std::str::FromStr;

use ::mongodb::bson;
use ::rug::Float;
use ::serde::{Deserialize, Serialize};
use ::types::casting::cast_f_from_txt;

use ::errors::ParseError;
use ::rpc::bot::Bot as RPCBot;
use ::rpc::entities::Exchanges;
use ::rpc::google::protobuf::Timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<bson::oid::ObjectId>,
  pub name: String,
  pub base_currency: String,
  pub exchange: Exchanges,
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
    exchange: Exchanges,
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
    let exchange = Exchanges::try_from(value.exchange).map_err(|err| {
      let mut err = err.clone();
      err.field = Some(String::from("exchange"));
      return err;
    });
    let trading_amount = value.trading_amount;
    let trading_amount = cast_f_from_txt("trading_amount", trading_amount)?;
    return Ok(Self {
      id: bson::oid::ObjectId::from_str(value.id.as_str()).ok(),
      name: value.name,
      base_currency: value.base_currency,
      exchange: exchange?,
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
      id: value.id.map(|id| id.to_hex()).unwrap_or("".to_string()),
      name: value.name,
      base_currency: value.base_currency,
      exchange: value.exchange as i32,
      created_at: value
        .created_at
        .map(|time| Timestamp::try_from(time.to_system_time()).ok())
        .flatten(),
      trading_amount: value.trading_amount.to_string(),
      condition: value.cond_ts,
    };
  }
}
