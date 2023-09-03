use ::std::time::SystemTimeError;

use ::err_derive::Error;
use ::redis::RedisError;

#[derive(Debug, Error)]
pub enum KVSError {
  #[error(display = "Redis error: {}", _0)]
  Redis(#[error(source)] RedisError),
  #[error(display = "System time error: {}", _0)]
  SystemTime(#[error(source)] SystemTimeError),
  #[error(display = "Specified key exists: {}", _0)]
  KeyExists(String),
  #[error(display = "Timestamp Error: {}", _0)]
  TimestampError(i64),
}

pub type KVSResult<T> = Result<T, KVSError>;
