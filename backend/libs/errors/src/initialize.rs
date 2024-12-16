use std::fmt::Debug;

use ::thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("Initialization Failed: {:?}", message)]
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
