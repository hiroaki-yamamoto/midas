use ::std::time::SystemTimeError;

use ::err_derive::Error;
use ::errors::{KVSError, PublishError, RespondError, UnknownExchangeError};
use ::kvs::redis::RedisError;
use ::mongodb::error::Error as DBErr;
use ::subscribe::natsJS::context::CreateStreamError as NatsCreateStreamError;

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
  #[error(display = "Nats Stream Creation Error: {}", _0)]
  NatsCreateStreamError(#[source] NatsCreateStreamError),
  #[error(display = "Nats Publish Error: {}", _0)]
  NatsPublishError(#[source] PublishError),
  #[error(display = "Respond Error: {}", _0)]
  RespondError(#[source] RespondError),
  #[error(display = "Unknown Exchange Error: {}", _0)]
  UnknownExchangeError(#[source] UnknownExchangeError),
}

pub type Result<T> = ::std::result::Result<T, Error>;
