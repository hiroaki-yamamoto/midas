use ::err_derive::Error;
use ::mongodb::bson::ser::Error as BSONEncErr;
use ::mongodb::error::Error as DBError;
use ::std::result::Result as StdResult;

use ::errors::ObjectNotFound;

#[derive(Debug, Clone, Error)]
pub enum BotInfoError {
  #[error(display = "{}", _0)]
  BSONEncErr(#[source] BSONEncErr),
  #[error(display = "{}", _0)]
  DBError(#[source] DBError),
  #[error(display = "{}", _0)]
  ObjectNotFound(#[source] ObjectNotFound),
}

pub type BotInfoResult<T> = StdResult<T, BotInfoError>;
