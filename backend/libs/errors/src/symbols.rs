use ::err_derive::Error;

use ::mongodb::error::Error as DBErr;
use ::reqwest::Error as ReqErr;

use crate::HTTPErrors;

#[derive(Debug, Error)]
pub enum SymbolFetchError {
  #[error(display = "Reqwest Error: {}", _0)]
  ReqErr(#[source] ReqErr),
  #[error(display = "HTTP Error: {}", _0)]
  HTTPErr(#[source] HTTPErrors),
  #[error(display = "Databse Error")]
  DBErr(#[source] DBErr),
}

pub type SymbolFetchResult<T> = Result<T, SymbolFetchError>;
