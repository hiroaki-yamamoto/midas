use ::std::io::Error as IOErr;

use ::err_derive::Error;
use ::reqwest::Error as ReqErr;
use ::serde_json::Error as JSONErr;

use crate::APIHeaderErrors;
use crate::HTTPErrors;
use crate::MaximumAttemptExceeded;
use crate::ParseError;
use crate::VecElementErrs;
use crate::WebsocketError;

#[derive(Debug, Error)]
pub enum NotificationError {
  #[error(display = "Multiple Float Parsing Errors: {}", _0)]
  MultipleParseErrors(#[source] VecElementErrs<ParseError>),
  #[error(display = "Parsing Error: {}", _0)]
  ParseError(#[source] ParseError),
  #[error(display = "IOError: {}", _0)]
  IOError(#[source] IOErr),
  #[error(display = "APIHeaderError: {}", _0)]
  APIHeaderError(#[source] APIHeaderErrors),
  #[error(display = "HttpError: {}", _0)]
  HttpErr(#[source] HTTPErrors),
  #[error(display = "WebSocket Error: {}", _0)]
  WebSocketErr(#[source] WebsocketError),
  #[error(display = "JSON Error: {}", _0)]
  JSONErr(#[source] JSONErr),
  #[error(display = "Maximum Attempt Exceeded: {}", _0)]
  MaximumAttemptExceeded(#[source] MaximumAttemptExceeded),
}

impl From<ReqErr> for NotificationError {
  fn from(value: ReqErr) -> Self {
    return value.into();
  }
}

pub type NotificationResult<T> = Result<T, NotificationError>;
