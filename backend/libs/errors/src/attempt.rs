use ::std::error::Error;
use ::std::fmt::{Debug, Display, Formatter, Result as FormatResult};

#[derive(Debug, Clone, Default)]
pub struct MaximumAttemptExceeded;

impl Display for MaximumAttemptExceeded {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Maximum retrieving count exceeded.");
  }
}

impl Error for MaximumAttemptExceeded {}
