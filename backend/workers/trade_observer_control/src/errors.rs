use ::std::time::SystemTimeError;

use ::err_derive::Error;
use ::kvs::redis::RedisError;

use ::errors::KVSError;

#[derive(Debug, Error)]
pub(crate) enum Error {
  #[error(display = "KVS error: {}", _0)]
  KVS(#[source] KVSError),
  #[error(display = "Redis error: {}", _0)]
  Redis(#[source] RedisError),
  #[error(display = "System Time Error: {}", _0)]
  SystemTime(#[source] SystemTimeError),
}

pub(crate) type Result<T> = ::std::result::Result<T, Error>;
