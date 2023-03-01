use ::err_derive::Error;
use ::reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use ::reqwest::Error as RequestError;
use ::url::Url;
use ::warp::reject::Reject;

use crate::MaximumAttemptExceeded;

#[derive(Debug, Clone, Error)]
#[error(
  display = "Status Failue (code: {}, text: {}, url: {:?})",
  code,
  text,
  url
)]
pub struct StatusFailure {
  pub url: Option<Url>,
  pub code: u16,
  pub text: String,
}

impl StatusFailure {
  pub fn new(url: Option<Url>, code: u16, text: String) -> Self {
    return Self { url, code, text };
  }
}

impl Reject for StatusFailure {}

#[derive(Debug, Error)]
pub enum HTTPErrors {
  #[error(display = "Invalid Header Value: {}", _0)]
  InvalidHeaderValue(#[source] InvalidHeaderValue),
  #[error(display = "Invalid Header Name {}", _0)]
  InvalidHeaderName(#[source] InvalidHeaderName),
  #[error(display = "Failed to send a request: {}", _0)]
  RequestFailure(#[source] RequestError),
  #[error(display = "Response Status Expectation Failure: {}", _0)]
  ResponseFailure(#[source] StatusFailure),
  #[error(display = "Round-robin Error")]
  RoundRobinExceeded(#[source] MaximumAttemptExceeded),
}

pub type HTTPResult<T> = Result<T, HTTPErrors>;
