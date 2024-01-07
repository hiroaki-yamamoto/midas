use ::std::io::Error as IOError;

use ::err_derive::Error;
use ::reqwest::Error as ReqErr;

use crate::keychain::KeyChainError;
use crate::status::HTTPErrors;

#[derive(Debug, Error)]
pub enum UserStreamError {
  #[error(display = "Request Error: {}", _0)]
  ReqErr(#[source] ReqErr),
  #[error(display = "IO Error: {}", _0)]
  IOError(#[source] IOError),
  #[error(display = "KeyChain Error: {}", _0)]
  KeyChainError(#[source] KeyChainError),
  #[error(display = "HTTP Error: {}", _0)]
  HTTPErrors(#[source] HTTPErrors),
}

pub type UserStreamResult<T> = Result<T, UserStreamError>;
