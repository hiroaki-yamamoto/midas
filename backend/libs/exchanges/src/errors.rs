use ::std::error::Error;
use ::std::fmt::{Debug, Display, Formatter, Result as FormatResult};

use ::http::StatusCode;

use ::url::Url;

#[derive(Debug, Clone, Default)]
pub struct MaximumAttemptExceeded;

impl Display for MaximumAttemptExceeded {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Maximum retrieving count exceeded.");
  }
}

impl Error for MaximumAttemptExceeded {}

unsafe impl Send for MaximumAttemptExceeded {}

#[derive(Debug, Clone)]
pub struct StatusFailure {
  pub url: Url,
  pub code: u16,
  pub text: String,
}

impl Display for StatusFailure {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Status Failure: {}", self);
  }
}
impl Error for StatusFailure {}

unsafe impl Send for StatusFailure {}

#[derive(Debug, Clone)]
pub struct EmptyError {
  pub field: String,
}

impl Display for EmptyError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Field {} is required, but it's empty", self.field);
  }
}

impl Error for EmptyError {}

unsafe impl Send for EmptyError {}

#[derive(Debug, Clone)]
pub struct WebsocketError {
  pub status: StatusCode,
}

impl Display for WebsocketError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(
      f,
      "Websocket Error: {} {}",
      self.status.as_u16(),
      self.status.as_str()
    );
  }
}

impl Error for WebsocketError {}
unsafe impl Send for WebsocketError {}

#[derive(Debug, Clone)]
pub struct ExecutionFailed {
  pub reason: String,
}

impl ExecutionFailed {
  pub fn new<T>(reason: T) -> Self
  where
    T: AsRef<str>,
  {
    return Self {
      reason: String::from(reason.as_ref()),
    };
  }
}

impl Display for ExecutionFailed {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Trade Execution Failed. Reason: {}", self.reason);
  }
}

impl Error for ExecutionFailed {}
