use ::async_nats::jetstream::context::CreateStreamError;
use ::err_derive::Error;
use ::mongodb::bson::ser::Error as BSONEncodeErr;
use ::mongodb::error::Error as DBErr;
use ::reqwest::header::InvalidHeaderName;
use ::reqwest::header::InvalidHeaderValue;
use ::reqwest::Error as ReqwestErr;
use ::serde_qs::Error as QueryError;
use ::std::convert::From;
use ::std::io::Error as IOError;

use crate::{
  APIHeaderErrors, HTTPErrors, KeyChainError, ObjectNotFound, ObserverError,
  ParseError, PositionError, StatusFailure,
};

#[derive(Debug, Clone, Error)]
#[error(display = "Trade Execution Failed. Reason: {}", reason)]
pub struct ExecutionFailed {
  pub reason: String,
}

impl ExecutionFailed {
  pub fn new(reason: &str) -> Self {
    return Self {
      reason: reason.into(),
    };
  }
}

#[derive(Debug, Error)]
pub enum ExecutionErrors {
  #[error(display = "Database Error: {}", _0)]
  EDBErr(#[source] DBErr),
  #[error(display = "API Header Error: {}", _0)]
  APIHeaderErrors(#[source] APIHeaderErrors),
  #[error(display = "BSON Serialization Error: {}", _0)]
  BSONEncodeErr(#[source] BSONEncodeErr),
  #[error(display = "HTTP Error: {}", _0)]
  HTTPError(#[source] HTTPErrors),
  #[error(display = "Query Encode Error: {}", _0)]
  QueryError(#[source] QueryError),
  #[error(display = "Execution Failure: {}", _0)]
  ExecutionFailure(#[source] ExecutionFailed),
  #[error(display = "Object Not Found: {}", _0)]
  ObjectNotFound(#[source] ObjectNotFound),
  #[error(display = "Cast Error: {}", _0)]
  ParseError(#[source] ParseError),
  #[error(display = "I/O Error: {}", _0)]
  IOError(#[source] IOError),
  #[error(display = "NATS Stream Creation Error: {}", _0)]
  NATSStreamCreationError(#[source] CreateStreamError),
  #[error(display = "Keychain Reference Error: {}", _0)]
  KeyChainError(#[source] KeyChainError),
  #[error(display = "Observer Error: {}", _0)]
  ObserverError(#[source] ObserverError),
  #[error(display = "Position Error: {}", _0)]
  PositionError(#[source] PositionError),
}

pub type ExecutionResult<T> = Result<T, ExecutionErrors>;

macro_rules! cast_enum_error {
  ($src_err: ty, $dest_err: expr) => {
    impl From<$src_err> for ExecutionErrors {
      fn from(err: $src_err) -> Self {
        return $dest_err(err).into();
      }
    }
  };
}

cast_enum_error!(StatusFailure, HTTPErrors::ResponseFailure);
cast_enum_error!(InvalidHeaderValue, HTTPErrors::InvalidHeaderValue);
cast_enum_error!(ReqwestErr, HTTPErrors::RequestFailure);
cast_enum_error!(InvalidHeaderName, HTTPErrors::InvalidHeaderName);
