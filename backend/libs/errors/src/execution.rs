use ::err_derive::Error;

#[derive(Debug, Clone, Error)]
#[error(display = "Trade Execution Failed. Reason: {}", reason)]
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
