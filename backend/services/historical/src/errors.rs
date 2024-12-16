use ::mongodb::error::Error as DBErr;
use ::serde_json::Error as DecodeError;
use ::thiserror::Error;

use ::errors::KVSError;
use ::subscribe::natsJS::context::CreateStreamError;

#[derive(Debug, Error)]
pub enum ServiceError {
  #[error("DB error: {}", _0)]
  DBError(#[from] DBErr),
  #[error("JSON decode error: {}", _0)]
  JSONDecodeError(#[from] DecodeError),
  #[error("Nats CreateStream error: {}", _0)]
  CreateStreamError(#[from] CreateStreamError),
  #[error("KVStore error: {}", _0)]
  KVSError(#[from] KVSError),
}

pub type ServiceResult<T> = Result<T, ServiceError>;
