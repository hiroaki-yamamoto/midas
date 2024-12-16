use ::std::io::Error as IOErr;

use ::mongodb::error::Error as DBErr;
use ::reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use ::serde_qs::Error as QSErr;
use ::thiserror::Error;

use crate::object::ObjectNotFound;
use crate::PublishError;
use ::async_nats::jetstream::context::CreateStreamError;

#[derive(Debug, Error)]
pub enum KeyChainError {
  #[error("Database Error: {}", _0)]
  DBErr(#[from] DBErr),
  #[error("IO Error (Perhaps Nats?): {}", _0)]
  IOErr(#[from] IOErr),
  #[error("pub/sub create stream error: {}", _0)]
  CreateStreamError(#[from] CreateStreamError),
  #[error("pub/sub publish error: {}", _0)]
  PublishError(#[from] PublishError),
  #[error("Key Not Found: {}", _0)]
  KeyNotFound(#[from] ObjectNotFound),
  #[error("Query String Encoding Error: {}", _0)]
  QSErr(#[from] QSErr),
  #[error("Invalid Header Name: {}", _0)]
  InvalidHeaderName(#[from] InvalidHeaderName),
  #[error("Invalid Header Value: {}", _0)]
  InvalidHeaderValue(#[from] InvalidHeaderValue),
}

pub type KeyChainResult<T> = Result<T, KeyChainError>;
