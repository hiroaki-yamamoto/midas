use ::err_derive::Error;

use ::mongodb::error::Error as DBErr;
use ::reqwest::header::{InvalidHeaderName, InvalidHeaderValue};

use crate::ObjectNotFound;

#[derive(Debug, Error)]
pub enum APIHeaderErrors {
  #[error(display = "Database Error: {}", _0)]
  DBErr(#[source] DBErr),
  #[error(display = "Invalid Header Name: {}", _0)]
  InvalidHeaderName(#[source] InvalidHeaderName),
  #[error(display = "Invalid Header Value: {}", _0)]
  InvalidHeaderValue(#[source] InvalidHeaderValue),
  #[error(display = "ObjectNotFound: {}", _0)]
  ObjectNotFound(#[source] ObjectNotFound),
}

pub type APIHeaderResult<T> = Result<T, APIHeaderErrors>;
