use ::std::error::Error;
use ::std::fmt::{Debug, Display, Formatter, Result as FormatResult};

use ::url::Url;

#[derive(Debug, Clone, Copy)]
pub struct MaximumAttemptExceeded;

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

#[derive(Debug, Clone, Copy)]
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
impl Error for StatusFailure {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

unsafe impl Send for StatusFailure {}

#[derive(Debug, Clone, Copy)]
pub struct EmptyError {
  pub field: String,
}

impl Display for EmptyError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Field {} is required, but it's empty", self.field);
  }
}

impl Error for EmptyError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

unsafe impl Send for EmptyError {}

#[derive(Debug, Clone, Copy)]
pub struct DeterminationFailed<T>
where
  T: Debug + Clone + Copy,
{
  pub field: String,
  pub additional_data: Option<T>,
}

impl<T> Display for DeterminationFailed<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(
      f,
      "Determination of {} failed. Additional Data: {:?}",
      self.field, self.additional_data
    );
  }
}

impl<T> Error for DeterminationFailed<T> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

unsafe impl<T> Send for DeterminationFailed<T> {}
