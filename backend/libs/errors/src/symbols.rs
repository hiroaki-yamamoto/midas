use ::thiserror::Error;

use ::mongodb::error::Error as DBErr;
use ::reqwest::Error as ReqErr;
use ::std::io::Error as IOError;
use ::url::ParseError as UrlParseError;

use crate::HTTPErrors;
use async_nats::jetstream::context::CreateStreamError;

#[derive(Debug, Error)]
pub enum SymbolFetchError {
  #[error("Reqwest Error: {}", _0)]
  ReqErr(#[from] ReqErr),
  #[error("HTTP Error: {}", _0)]
  HTTPErr(#[from] HTTPErrors),
  #[error("Databse Error")]
  DBErr(#[from] DBErr),
  #[error("IO Error (Perhaps from broker?): {}", _0)]
  IOError(#[from] IOError),
  #[error("NATS Create Stream Error: {}", _0)]
  NatsCreateStreamError(#[from] CreateStreamError),
  #[error("URL Parse Error: {}", _0)]
  UrlParseError(#[from] UrlParseError),
}

pub type SymbolFetchResult<T> = Result<T, SymbolFetchError>;
