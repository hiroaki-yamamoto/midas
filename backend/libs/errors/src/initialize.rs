use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FormatResult};

#[derive(Debug, Clone)]
pub struct InitError<T>
where
  T: AsRef<str> + Clone,
{
  message: Option<T>,
}

impl<T> InitError<T>
where
  T: AsRef<str> + Clone,
{
  pub fn new(msg: Option<T>) -> Self {
    return Self { message: msg };
  }
}

impl<T> Display for InitError<T>
where
  T: AsRef<str> + Clone,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return match &self.message {
      None => write!(f, "Initialization Failed"),
      Some(msg) => write!(f, "Initialization Failed: {}", msg.as_ref()),
    };
  }
}

impl<T> Error for InitError<T> where T: AsRef<str> + Debug + Clone {}
