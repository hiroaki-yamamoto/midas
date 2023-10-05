use ::err_derive::Error;

#[derive(Debug, Error)]
#[error(display = "Timeout")]
pub struct TimeoutError;

pub type TimeoutResult<T> = Result<T, TimeoutError>;
