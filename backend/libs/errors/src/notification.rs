use ::std::io::Error as IOErr;

use ::async_nats::jetstream::context::CreateStreamError;

use ::reqwest::Error as ReqErr;
use ::serde_json::Error as JSONErr;
use ::thiserror::Error;
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
  #[error("Multiple Float Parsing Errors: {}", _0)]
  MultipleParseErrors(#[from] VecElementErrs<ParseError>),
  #[error("Parsing Error: {}", _0)]
  ParseError(#[from] ParseError),
  #[error("IOError: {}", _0)]
  IOError(#[from] IOErr),
  #[error("APIHeaderError: {}", _0)]
  APIHeaderError(#[from] APIHeaderErrors),
  #[error("HttpError: {}", _0)]
  HttpErr(#[from] HTTPErrors),
  #[error("WebSocket Error: {}", _0)]
  WebSocketErr(#[from] WebsocketError),
  #[error("JSON Error: {}", _0)]
  JSONErr(#[from] JSONErr),
  #[error("Maximum Attempt Exceeded: {}", _0)]
  MaximumAttemptExceeded(#[from] MaximumAttemptExceeded),
  #[error("Nats Stream Creation Error: {}", _0)]
  CreateStreamError(#[from] CreateStreamError),
  #[error("Nats Publish Error: {}", _0)]
  PublishError(#[from] PublishError),
  #[error("Nats Consumer Error: {}", _0)]
  ConsumerError(#[from] ConsumerError),
  #[error("URL Parse Error: {}", _0)]
  UrlParseError(#[from] UrlParseErr),
  #[error("User Stream Error: {}", _0)]
  UserStreamError(#[from] UserStreamError),
}

impl From<ReqErr> for NotificationError {
  fn from(value: ReqErr) -> Self {
    return value.into();
  }
}

pub type NotificationResult<T> = Result<T, NotificationError>;
