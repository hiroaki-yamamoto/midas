use ::mongodb::bson::ser::Error as BSONEncodeErr;
use ::mongodb::bson::Bson;
use ::mongodb::error::Error as DBErr;
use ::thiserror::Error;

use crate::object::ObjectNotFound;

#[derive(Clone, Debug, Error)]
pub enum PositionError {
  #[error("Database error: {}", _0)]
  DBError(#[from] DBErr),
  #[error("BSON Encode Error: {}", _0)]
  BSONEncodeErr(#[from] BSONEncodeErr),
  #[error("BSON Casting Failed: {}", _0)]
  BSONCastFailed(Bson),
  #[error("Object not found: {}", _0)]
  ObjectNotFound(#[from] ObjectNotFound),
}

pub type PositionResult<T> = ::std::result::Result<T, PositionError>;
