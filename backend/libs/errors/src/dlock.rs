use crate::kvs::KVSError;
use ::err_derive::Error;

#[derive(Debug, Error)]
pub enum DLockError {
  #[error(display = "DLock Failure (KVS Error): {}", _0)]
  KVSError(#[source] KVSError),
  #[error(display = "Cast Failure: {})", _0)]
  CastFailure(&'static str),
}

pub type DLockResult<T> = Result<T, DLockError>;
