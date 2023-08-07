use ::err_derive::Error;
use ::redis::RedisError;

#[derive(Debug, Error)]
pub enum DLockError {
  #[error(display = "DLock Failure (Redis Error): {}", _0)]
  RedisError(#[source] RedisError),
  #[error(display = "Cast Failure: {})", _0)]
  CastFailure(&'static str),
}

pub type DLockResult<T> = Result<T, DLockError>;
