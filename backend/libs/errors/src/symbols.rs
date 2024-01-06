use ::err_derive::Error;

use ::mongodb::error::Error as DBErr;
use ::reqwest::Error as ReqErr;
use ::std::io::Error as IOError;
use ::url::ParseError as UrlParseError;

use crate::HTTPErrors;
use async_nats::jetstream::context::CreateStreamError;

#[derive(Debug, Error)]
pub enum SymbolFetchError {
  #[error(display = "Reqwest Error: {}", _0)]
  ReqErr(#[source] ReqErr),
  #[error(display = "HTTP Error: {}", _0)]
  HTTPErr(#[source] HTTPErrors),
  #[error(display = "Databse Error")]
  DBErr(#[source] DBErr),
  #[error(display = "IO Error (Perhaps from broker?): {}", _0)]
  IOError(#[source] IOError),
  #[error(display = "NATS Create Stream Error: {}", _0)]
  NatsCreateStreamError(#[source] CreateStreamError),
  #[error(display = "URL Parse Error: {}", _0)]
  UrlParseError(#[source] UrlParseError),
}

pub type SymbolFetchResult<T> = Result<T, SymbolFetchError>;
