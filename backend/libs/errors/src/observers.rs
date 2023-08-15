use ::std::io::Error as IOErr;
use ::std::string::FromUtf8Error;

use ::err_derive::Error;
use ::mongodb::error::Error as DBErr;
use ::serde_json::Error as JSONErr;

use crate::pubsub::ConsumerError;
use crate::EmptyError;
use crate::InitError;
use crate::MaximumAttemptExceeded;

#[derive(Debug, Error)]
pub enum ObserverError {
  #[error(display = "Maximum Attempt Exceeded")]
  MaximumAttemptExceeded(#[source] MaximumAttemptExceeded),
  #[error(display = "JSONify Failure: {}", _0)]
  JSONErr(#[source] JSONErr),
  #[error(display = "UTF8 Parse Error: {}", _0)]
  UTF8ParseErr(#[source] FromUtf8Error),
  #[error(display = "Empty Err: {}", _0)]
  EmptyError(#[source] EmptyError),
  #[error(display = "IOError: {}", _0)]
  IOErr(#[source] IOErr),
  #[error(display = "Initialization Error: {}", _0)]
  InitErr(#[source] InitError),
  #[error(display = "DB Error: {}", _0)]
  DBErr(#[source] DBErr),
  #[error(display = "NATS consumer error: {}", _0)]
  ConsumerError(#[source] ConsumerError),
}

pub type ObserverResult<T> = Result<T, ObserverError>;
