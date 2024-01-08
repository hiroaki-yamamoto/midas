use ::std::io::Error as IOErr;

use ::async_nats::jetstream::context::CreateStreamError;

use ::err_derive::Error;
use ::reqwest::Error as ReqErr;
use ::serde_json::Error as JSONErr;
use ::url::ParseError as UrlParseErr;

use crate::user_stream::UserStreamError;
use crate::APIHeaderErrors;
use crate::HTTPErrors;
use crate::MaximumAttemptExceeded;
use crate::ParseError;
use crate::VecElementErrs;
use crate::WebsocketError;
use crate::{ConsumerError, PublishError};

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
  #[error(display = "Nats Stream Creation Error: {}", _0)]
  CreateStreamError(#[source] CreateStreamError),
  #[error(display = "Nats Publish Error: {}", _0)]
  PublishError(#[source] PublishError),
  #[error(display = "Nats Consumer Error: {}", _0)]
  ConsumerError(#[source] ConsumerError),
  #[error(display = "URL Parse Error: {}", _0)]
  UrlParseError(#[source] UrlParseErr),
  #[error(display = "User Stream Error: {}", _0)]
  UserStreamError(#[source] UserStreamError),
}

impl From<ReqErr> for NotificationError {
  fn from(value: ReqErr) -> Self {
    return value.into();
  }
}

pub type NotificationResult<T> = Result<T, NotificationError>;
