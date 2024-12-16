use ::thiserror::Error;

use ::mongodb::error::Error as DBErr;
use ::reqwest::header::{InvalidHeaderName, InvalidHeaderValue};

use crate::ObjectNotFound;

#[derive(Debug, Error)]
pub enum APIHeaderErrors {
  #[error("Database Error: {}", _0)]
  DBErr(#[from] DBErr),
  #[error("Invalid Header Name: {}", _0)]
  InvalidHeaderName(#[from] InvalidHeaderName),
  #[error("Invalid Header Value: {}", _0)]
  InvalidHeaderValue(#[from] InvalidHeaderValue),
  #[error("ObjectNotFound: {}", _0)]
  ObjectNotFound(#[from] ObjectNotFound),
}

pub type APIHeaderResult<T> = Result<T, APIHeaderErrors>;
