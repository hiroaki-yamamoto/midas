use ::std::io::Error as IOErr;

use ::err_derive::Error;
use ::mongodb::error::Error as DBErr;
use ::reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use ::serde_qs::Error as QSErr;

use crate::object::ObjectNotFound;
use crate::PublishError;
use ::async_nats::jetstream::context::CreateStreamError;

#[derive(Debug, Error)]
pub enum KeyChainError {
  #[error(display = "Database Error: {}", _0)]
  DBErr(#[source] DBErr),
  #[error(display = "IO Error (Perhaps Nats?): {}", _0)]
  IOErr(#[source] IOErr),
  #[error(display = "pub/sub create stream error: {}", _0)]
  CreateStreamError(#[source] CreateStreamError),
  #[error(display = "pub/sub publish error: {}", _0)]
  PublishError(#[source] PublishError),
  #[error(display = "Key Not Found: {}", _0)]
  KeyNotFound(#[source] ObjectNotFound),
  #[error(display = "Query String Encoding Error: {}", _0)]
  QSErr(#[source] QSErr),
  #[error(display = "Invalid Header Name: {}", _0)]
  InvalidHeaderName(#[source] InvalidHeaderName),
  #[error(display = "Invalid Header Value: {}", _0)]
  InvalidHeaderValue(#[source] InvalidHeaderValue),
}

pub type KeyChainResult<T> = Result<T, KeyChainError>;
