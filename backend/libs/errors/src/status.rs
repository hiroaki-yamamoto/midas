use ::std::error::Error;
use ::std::fmt::{Debug, Display, Formatter, Result as FormatResult};

use ::url::Url;

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
