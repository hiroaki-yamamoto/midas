use ::mongodb::bson::ser::Error as BSONEncErr;
use ::mongodb::error::Error as DBError;
use ::std::result::Result as StdResult;
use ::thiserror::Error;

use ::errors::ObjectNotFound;

#[derive(Debug, Clone, Error)]
pub enum BotInfoError {
  #[error("{}", _0)]
  BSONEncErr(#[from] BSONEncErr),
  #[error("{}", _0)]
  DBError(#[from] DBError),
  #[error("{}", _0)]
  ObjectNotFound(#[from] ObjectNotFound),
}

pub type BotInfoResult<T> = StdResult<T, BotInfoError>;
