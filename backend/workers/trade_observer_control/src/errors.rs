use ::std::time::SystemTimeError;

use ::err_derive::Error;
use ::kvs::redis::RedisError;
use ::mongodb::error::Error as DBErr;

use ::errors::KVSError;

#[derive(Debug, Error)]
pub enum Error {
  #[error(display = "KVS error: {}", _0)]
  KVS(#[source] KVSError),
  #[error(display = "Redis error: {}", _0)]
  Redis(#[source] RedisError),
  #[error(display = "System Time Error: {}", _0)]
  SystemTime(#[source] SystemTimeError),
  #[error(display = "DB Error: {}", _0)]
  DB(#[source] DBErr),
}

pub type Result<T> = ::std::result::Result<T, Error>;
