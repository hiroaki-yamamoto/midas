use ::std::convert::From;
use ::std::io::Error as IOError;

use ::async_nats::jetstream::context::CreateStreamError;
use ::mongodb::bson::ser::Error as BSONEncodeErr;
use ::mongodb::error::Error as DBErr;
use ::reqwest::header::InvalidHeaderName;
use ::reqwest::header::InvalidHeaderValue;
use ::reqwest::Error as ReqwestErr;
use ::rug::Float;
use ::serde_qs::Error as QueryError;
use ::thiserror::Error;
use ::url::ParseError as URLParseErr;

use crate::{
  APIHeaderErrors, HTTPErrors, KeyChainError, ObjectNotFound, ObserverError,
  ParseError, PositionError, StatusFailure,
};

#[derive(Debug, Clone, Error)]
#[error("Trade Execution Failed. Reason: {}", reason)]
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
  #[error("Database Error: {}", _0)]
  EDBErr(#[from] DBErr),
  #[error("API Header Error: {}", _0)]
  APIHeaderErrors(#[from] APIHeaderErrors),
  #[error("BSON Serialization Error: {}", _0)]
  BSONEncodeErr(#[from] BSONEncodeErr),
  #[error("HTTP Error: {}", _0)]
  HTTPError(#[from] HTTPErrors),
  #[error("Query Encode Error: {}", _0)]
  QueryError(#[from] QueryError),
  #[error("Execution Failure: {}", _0)]
  ExecutionFailure(#[from] ExecutionFailed),
  #[error("Object Not Found: {}", _0)]
  ObjectNotFound(#[from] ObjectNotFound),
  #[error("Cast Error: {}", _0)]
  ParseError(#[from] ParseError),
  #[error("I/O Error: {}", _0)]
  IOError(#[from] IOError),
  #[error("NATS Stream Creation Error: {}", _0)]
  NATSStreamCreationError(#[from] CreateStreamError),
  #[error("Keychain Reference Error: {}", _0)]
  KeyChainError(#[from] KeyChainError),
  #[error("Observer Error: {}", _0)]
  ObserverError(#[from] ObserverError),
  #[error("Position Error: {}", _0)]
  PositionError(#[from] PositionError),
  #[error("The order already filled")]
  OrderFilled,
  #[error("Invalid Qty {}", _0)]
  InvalidQty(Float),
  #[error("URL parse error: {}", _0)]
  URLParseErr(#[from] URLParseErr),
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
