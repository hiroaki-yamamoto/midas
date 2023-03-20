use ::err_derive::Error;

use ::mongodb::error::Error as DBErr;
use ::std::io::Error as IOErr;

#[derive(Debug, Error)]
pub enum KeyChainError {
  #[error(display = "Database Error: {}", _0)]
  DBErr(#[source] DBErr),
  #[error(display = "IO Error (Perhaps Nats?): {}", _0)]
  IOErr(#[source] IOErr),
}

pub type KeyChainResult<T> = Result<T, KeyChainError>;
