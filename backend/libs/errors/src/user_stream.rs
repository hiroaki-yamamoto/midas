use ::std::io::Error as IOError;

use ::err_derive::Error;
use ::reqwest::Error as ReqErr;

#[derive(Debug, Error)]
pub enum UserStreamError {
  #[error(display = "Request Error: {}", _0)]
  ReqErr(#[source] ReqErr),
  #[error(display = "IO Error: {}", _0)]
  IOError(#[source] IOError),
}

pub type UserStreamResult<T> = Result<T, UserStreamError>;
