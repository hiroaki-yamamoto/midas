use ::err_derive::Error;

use ::errors::KVSError;
use ::subscribe::natsJS::context::CreateStreamError;

#[derive(Debug, Error)]
pub enum ServiceError {
  #[error(display = "Nats CreateStream error: {}", _0)]
  CreateStreamError(#[source] CreateStreamError),
  #[error(display = "KVStore error: {}", _0)]
  KVSError(#[source] KVSError),
}

pub type ServiceResult<T> = Result<T, ServiceError>;
