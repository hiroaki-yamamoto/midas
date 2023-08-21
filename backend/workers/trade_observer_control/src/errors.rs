use ::err_derive::Error;
use ::kvs::redis::RedisError;

#[derive(Debug, Error)]
pub(crate) enum Error {
  #[error(display = "Redis error: {}", _0)]
  Redis(#[source] RedisError),
}

pub(crate) type Result<T> = ::std::result::Result<T, Error>;
