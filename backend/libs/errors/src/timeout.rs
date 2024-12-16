use ::thiserror::Error;

#[derive(Debug, Error)]
#[error("Timeout")]
pub struct TimeoutError;

pub type TimeoutResult<T> = Result<T, TimeoutError>;
