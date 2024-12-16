use ::std::io::Error as IOError;

use ::reqwest::Error as ReqErr;
use ::thiserror::Error;

use crate::keychain::KeyChainError;
use crate::status::HTTPErrors;

#[derive(Debug, Error)]
pub enum UserStreamError {
  #[error("Request Error: {}", _0)]
  ReqErr(#[from] ReqErr),
  #[error("IO Error: {}", _0)]
  IOError(#[from] IOError),
  #[error("KeyChain Error: {}", _0)]
  KeyChainError(#[from] KeyChainError),
  #[error("HTTP Error: {}", _0)]
  HTTPErrors(#[from] HTTPErrors),
}

pub type UserStreamResult<T> = Result<T, UserStreamError>;
