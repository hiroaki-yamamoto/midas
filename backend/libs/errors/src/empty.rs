use ::thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("Field {} is required, but it's empty", field)]
pub struct EmptyError {
  pub field: String,
}
