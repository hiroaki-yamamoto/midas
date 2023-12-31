use ::err_derive::Error;
use ::mongodb::bson::ser::Error as BSONEncodeErr;
use ::mongodb::error::Error as DBErr;

#[derive(Debug, Error)]
pub enum PositionError {
  #[error(display = "Database error: {}", _0)]
  DBError(#[error(source)] DBErr),
  #[error(display = "BSON Encode Error: {}", _0)]
  BSONEncodeErr(#[error(source)] BSONEncodeErr),
}

pub type PositionResult<T> = ::std::result::Result<T, PositionError>;
