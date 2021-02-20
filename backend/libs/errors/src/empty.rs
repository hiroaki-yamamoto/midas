use ::std::error::Error;
use ::std::fmt::{Debug, Display, Formatter, Result as FormatResult};

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
