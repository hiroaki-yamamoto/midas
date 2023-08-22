use ::async_nats::jetstream::kv::{EntryError, PutError};
use ::err_derive::Error;
use ::rmp_serde::decode::Error as DecodeError;

#[derive(Debug, Error)]
pub enum NatsKVSError {
  #[error(display = "NatsKVSError::EntryError: {}", _0)]
  EntryError(#[error(source)] EntryError),
  #[error(display = "NatsKVSError::PutError: {}", _0)]
  PutError(#[error(source)] PutError),
  #[error(display = "NatsKVSError::DecodeError: {}", _0)]
  DecodeError(#[error(source)] DecodeError),
  #[error(display = "NatsKVSError::NoValue")]
  NoValue,
}

pub type NatsKVSResult<T> = Result<T, NatsKVSError>;
