use ::std::convert::TryFrom;

use ::mongodb::bson::{oid::ObjectId, DateTime};
use ::rug::Float;
use ::serde::{Deserialize, Serialize};
use ::types::casting::cast_f_from_txt;
use ::types::{stateful_setter, DateTime as UTCDateTime};

use ::errors::ParseError;
use ::rpc::position::Position as RPCPosition;
use ::rpc::position_status::PositionStatus as RPCPositionStatus;

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
  #[serde(rename = "_id")]
  id: ObjectId,
  bot_id: ObjectId,
  entry_at: DateTime,
  entry_price: Float,
  amount: Float,
  exit_at: Option<DateTime>,
  exit_price: Option<Float>,
  symbol: String,
}

impl Position {
  pub fn new(
    bot_id: ObjectId,
    symbol: String,
    entry_at: DateTime,
    entry_price: Float,
    amount: Float,
  ) -> Self {
    return Self {
      id: ObjectId::new(),
      bot_id,
      symbol,
      entry_at,
      entry_price,
      amount,
      exit_at: None,
      exit_price: None,
    };
  }

  stateful_setter!(exit_at, Option<DateTime>);
  stateful_setter!(exit_price, Option<Float>);
}

impl From<Position> for RPCPosition {
  fn from(value: Position) -> Self {
    let entry_at: UTCDateTime = value.entry_at.to_chrono();
    let exit_at: Option<UTCDateTime> =
      value.exit_at.map(|exit_at| exit_at.to_chrono());
    let status: RPCPositionStatus = if exit_at.is_some() {
      RPCPositionStatus::CLOSE
    } else {
      RPCPositionStatus::OPEN
    };
    return RPCPosition {
      id: value.id.to_string(),
      bot_id: value.bot_id.to_string(),
      entry_at: Box::new(entry_at.into()),
      entry_price: value.entry_price.to_string(),
      amount: value.amount.to_string(),
      exit_at: exit_at.map(|exit_at| Box::new(exit_at.into())),
      exit_price: value.exit_price.map(|exit_price| exit_price.to_string()),
      symbol: value.symbol,
      status: Box::new(status),
    };
  }
}

impl TryFrom<RPCPosition> for Position {
  type Error = ParseError;

  fn try_from(value: RPCPosition) -> Result<Self, Self::Error> {
    let id = ObjectId::parse_str(&value.id)
      .map_err(ParseError::raise_parse_err("id", &value.id))?;
    let bot_id = ObjectId::parse_str(&value.bot_id)
      .map_err(ParseError::raise_parse_err("bot_id", &value.bot_id))?;
    let entry_at: UTCDateTime = value.entry_at.as_ref().try_into().map_err(
      ParseError::raise_parse_err("entry_at", &value.entry_at.to_string()),
    )?;
    let exit_at: Option<DateTime> = value
      .exit_at
      .as_ref()
      .map(|exit_at| {
        let exit_at: Result<UTCDateTime, ParseError> =
          exit_at.as_ref().try_into();
        return exit_at.map(|exit_at| exit_at.into());
      })
      .transpose()
      .map_err(ParseError::raise_parse_err(
        "exit_at",
        format!("{:?}", value.exit_at).as_str(),
      ))?;
    let exit_price: Option<Float> = value
      .exit_price
      .as_ref()
      .map(|exit_price| cast_f_from_txt("exit_price", exit_price))
      .transpose()
      .map_err(ParseError::raise_parse_err(
        "exit_price",
        format!("{:?}", value.exit_price).as_str(),
      ))?;
    let mut entity = Self::new(
      bot_id,
      value.symbol,
      DateTime::from_chrono(entry_at),
      cast_f_from_txt("entry_price", &value.entry_price)?,
      cast_f_from_txt("amount", &value.amount)?,
    )
    .exit_at(exit_at)
    .exit_price(exit_price);
    entity.id = id;
    return Ok(entity);
  }
}
