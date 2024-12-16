use ::mongodb::error::Error as DBErr;
use ::reqwest::Error as ReqErr;
use ::thiserror::Error;
use ::url::ParseError as URLParseErr;

use crate::HTTPErrors;
use crate::MaximumAttemptExceeded;
use crate::UnknownExchangeError;
use crate::ValidationErr;

#[derive(Debug, Error)]
#[error("HistoryFetchError")]
pub enum FetchErr {
  #[error("Reqwest Err: {}", _0)]
  ReqwestErr(#[from] ReqErr),
  #[error("Invalid field: {}", _0)]
  ValidationErr(#[from] ValidationErr),
  #[error("Rest Client Error: {}", _0)]
  HTTPErr(#[from] HTTPErrors),
  #[error("Maximum Attempt Exceeded: {}", _0)]
  MaximumAttemptExceeded(#[from] MaximumAttemptExceeded),
  #[error("URL Parse Error: {}", _0)]
  URLParseErr(#[from] URLParseErr),
}

#[derive(Debug, Error)]
pub enum WriterErr {
  #[error("Unknown Exchange: {}", _0)]
  UnknownExchange(#[from] UnknownExchangeError),
  #[error("Database Error: {}", _0)]
  DBErr(#[from] DBErr),
}

pub type FetchResult<T> = Result<T, FetchErr>;
pub type WriterResult<T> = Result<T, WriterErr>;
