use ::err_derive::Error;
use ::mongodb::bson::ser::Error as BSONEncodeErr;
use ::mongodb::bson::Bson;
use ::mongodb::error::Error as DBErr;

use crate::object::ObjectNotFound;

#[derive(Debug, Error)]
pub enum PositionError {
  #[error(display = "Database error: {}", _0)]
  DBError(#[error(source)] DBErr),
  #[error(display = "BSON Encode Error: {}", _0)]
  BSONEncodeErr(#[error(source)] BSONEncodeErr),
  #[error(display = "BSON Casting Failed: {}", _0)]
  BSONCastFailed(Bson),
  #[error(display = "Object not found: {}", _0)]
  ObjectNotFound(#[error(source)] ObjectNotFound),
}

pub type PositionResult<T> = ::std::result::Result<T, PositionError>;
