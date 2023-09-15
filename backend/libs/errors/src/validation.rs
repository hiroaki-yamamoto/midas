use ::err_derive::Error;
use ::serde::Serialize;

#[derive(Clone, Debug, Error, Serialize)]
#[error(display = "Validation Failed")]
pub struct ValidationErr {
  pub field: String,
  pub reason: String,
}

impl ValidationErr {
  pub fn new(field: &str, reason: &str) -> Self {
    return Self {
      field: field.into(),
      reason: reason.into(),
    };
  }
}
