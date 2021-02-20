use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FormatResult};

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
