use ::redis::RedisError;
use ::thiserror::Error;

#[derive(Debug, Error)]
pub enum DLockError {
  #[error("DLock Failure (Redis Error): {}", _0)]
  RedisError(#[from] RedisError),
  #[error("Cast Failure: {})", _0)]
  CastFailure(&'static str),
}

pub type DLockResult<T> = Result<T, DLockError>;
