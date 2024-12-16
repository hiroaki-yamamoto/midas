use ::std::time::SystemTimeError;

use ::redis::RedisError;
use ::thiserror::Error;

#[derive(Debug, Error)]
pub enum KVSError {
  #[error("Redis error: {}", _0)]
  Redis(#[from] RedisError),
  #[error("System time error: {}", _0)]
  SystemTime(#[from] SystemTimeError),
  #[error("Specified key exists: {}", _0)]
  KeyExists(String),
  #[error("Timestamp Error: {}", _0)]
  TimestampError(i64),
}

pub type KVSResult<T> = Result<T, KVSError>;
