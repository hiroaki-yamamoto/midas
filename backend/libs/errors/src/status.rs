use ::reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use ::reqwest::Error as RequestError;
use ::thiserror::Error;
use ::warp::reject::Reject;

use crate::MaximumAttemptExceeded;

#[derive(Debug, Clone, Error)]
#[error("Status Failue (code: {}, text: {}, url: {:?})", code, text, url)]
pub struct StatusFailure {
  pub url: Option<String>,
  pub code: u16,
  pub text: String,
}

impl StatusFailure {
  pub fn new(url: Option<String>, code: u16, text: String) -> Self {
    return Self { url, code, text };
  }
}

impl Reject for StatusFailure {}

#[derive(Debug, Error)]
pub enum HTTPErrors {
  #[error("Invalid Header Value: {}", _0)]
  InvalidHeaderValue(#[from] InvalidHeaderValue),
  #[error("Invalid Header Name {}", _0)]
  InvalidHeaderName(#[from] InvalidHeaderName),
  #[error("Failed to send a request: {}", _0)]
  RequestFailure(#[from] RequestError),
  #[error("Response Status Expectation Failure: {}", _0)]
  ResponseFailure(#[from] StatusFailure),
  #[error("Round-robin Error")]
  RoundRobinExceeded(#[from] MaximumAttemptExceeded),
}

pub type HTTPResult<T> = Result<T, HTTPErrors>;
