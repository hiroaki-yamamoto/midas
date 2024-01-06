use ::err_derive::Error;
use ::mongodb::error::Error as DBErr;
use ::reqwest::Error as ReqErr;
use ::url::ParseError as URLParseErr;

use crate::HTTPErrors;
use crate::MaximumAttemptExceeded;
use crate::UnknownExchangeError;
use crate::ValidationErr;

#[derive(Debug, Error)]
#[error(display = "HistoryFetchError")]
pub enum FetchErr {
  #[error(display = "Reqwest Err: {}", _0)]
  ReqwestErr(#[source] ReqErr),
  #[error(display = "Invalid field: {}", _0)]
  ValidationErr(#[source] ValidationErr),
  #[error(display = "Rest Client Error: {}", _0)]
  HTTPErr(#[source] HTTPErrors),
  #[error(display = "Maximum Attempt Exceeded: {}", _0)]
  MaximumAttemptExceeded(#[source] MaximumAttemptExceeded),
  #[error(display = "URL Parse Error: {}", _0)]
  URLParseErr(#[source] URLParseErr),
}

#[derive(Debug, Error)]
pub enum WriterErr {
  #[error(display = "Unknown Exchange: {}", _0)]
  UnknownExchange(#[source] UnknownExchangeError),
  #[error(display = "Database Error: {}", _0)]
  DBErr(#[source] DBErr),
}

pub type FetchResult<T> = Result<T, FetchErr>;
pub type WriterResult<T> = Result<T, WriterErr>;
