use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FormatResult};

use ::url::Url;

#[derive(Debug, Default)]
pub(crate) struct MaximumAttemptExceeded;

impl Display for MaximumAttemptExceeded {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Maximum retrieving count exceeded.");
  }
}

impl Error for MaximumAttemptExceeded {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

unsafe impl Send for MaximumAttemptExceeded {}

#[derive(Debug)]
pub(crate) struct StatusFailure {
  pub url: Url,
  pub code: u16,
  pub text: String,
}

impl Display for StatusFailure {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Status Failure: {}", self);
  }
}
impl Error for StatusFailure {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

unsafe impl Send for StatusFailure {}
