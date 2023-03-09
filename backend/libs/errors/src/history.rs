use ::err_derive::Error;
use ::reqwest::Error as ReqErr;

use crate::HTTPErrors;
use crate::MaximumAttemptExceeded;
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
}

pub type FetchResult<T> = Result<T, FetchErr>;
