use ::serde::Serialize;
use ::thiserror::Error;

#[derive(Clone, Debug, Error, Serialize)]
#[error("Validation Failed")]
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
