use ::err_derive::Error;
use ::mongodb::error::Error as DBErr;
use ::serde_qs::Error as QueryError;

use ::errors::{ExecutionFailed, HTTPErrors, ObjectNotFound};

#[derive(Debug, Error)]
pub enum ExecutionErrors {
  #[error(display = "Database Error: {}", _0)]
  EDBErr(#[source] DBErr),
  #[error(display = "HTTP Error: {}", _0)]
  HTTPError(#[source] HTTPErrors),
  #[error(display = "Query Encode Error: {}", _0)]
  QueryError(#[source] QueryError),
  #[error(display = "Execution Failure: {}", _0)]
  ExecutionFailure(#[source] ExecutionFailed),
  #[error(display = "Object Not Found: {}", _0)]
  ObjectNotFound(#[source] ObjectNotFound),
}

pub type ExecutionResult<T> = Result<T, ExecutionErrors>;
