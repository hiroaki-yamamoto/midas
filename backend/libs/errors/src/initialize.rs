use std::fmt::Debug;

use ::err_derive::Error;

#[derive(Debug, Clone, Error)]
#[error(display = "Initialization Failed: {:?}", message)]
pub struct InitError {
  message: Option<String>,
}

impl InitError {
  pub fn new<T>(message: T) -> Self
  where
    T: Into<Option<String>>,
  {
    return Self {
      message: message.into(),
    };
  }
}
