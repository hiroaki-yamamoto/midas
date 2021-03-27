use ::err_derive::Error;

#[derive(Debug, Clone, Error)]
#[error(display = "Field {} is required, but it's empty", field)]
pub struct EmptyError {
  pub field: String,
}
