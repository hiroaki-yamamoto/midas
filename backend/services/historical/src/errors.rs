use ::err_derive::Error;
use ::mongodb::error::Error as DBErr;
use ::serde_json::Error as DecodeError;

use ::errors::KVSError;
use ::subscribe::natsJS::context::CreateStreamError;

#[derive(Debug, Error)]
pub enum ServiceError {
  #[error(display = "DB error: {}", _0)]
  DBError(#[source] DBErr),
  #[error(display = "JSON decode error: {}", _0)]
  JSONDecodeError(#[source] DecodeError),
  #[error(display = "Nats CreateStream error: {}", _0)]
  CreateStreamError(#[source] CreateStreamError),
  #[error(display = "KVStore error: {}", _0)]
  KVSError(#[source] KVSError),
}

pub type ServiceResult<T> = Result<T, ServiceError>;
