use ::err_derive::Error;

use ::mongodb::error::Error as DBErr;
use ::std::io::Error as IOErr;

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
}

pub type KeyChainResult<T> = Result<T, KeyChainError>;
