use ::err_derive::Error;
use ::serde::Serialize;

#[derive(Clone, Debug, Error, Serialize)]
#[error(display = "Validation Failed")]
pub struct ValidationErr {
  pub field: String,
  pub reason: String,
}

impl ValidationErr {
  pub fn new<S, T>(field: S, reason: T) -> Self
  where
    S: AsRef<str>,
    T: AsRef<str>,
  {
    return Self {
      field: field.as_ref().into(),
      reason: reason.as_ref().into(),
    };
  }
}
