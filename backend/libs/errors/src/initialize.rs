use std::fmt::Debug;

use ::err_derive::Error;

#[derive(Debug, Clone, Error)]
#[error(display = "Initialization Failed: {:?}", message)]
pub struct InitError<T>
where
  T: AsRef<str> + Clone + Debug,
{
  message: Option<T>,
}

impl<T> InitError<T>
where
  T: AsRef<str> + Clone + Debug,
{
  pub fn new(msg: Option<T>) -> Self {
    return Self { message: msg };
  }
}
