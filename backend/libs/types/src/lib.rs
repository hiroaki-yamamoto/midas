mod entities;

use ::std::error::Error;
use ::std::result::Result as StdResult;
use ::tonic::Status as TonicStatus;
use ::url::{ParseError, Url};

pub use self::entities::Status;

pub type Result<T> = StdResult<T, TonicStatus>;
pub type ParseURLResult = StdResult<Url, ParseError>;
pub type GenericResult<T> = StdResult<T, Box<dyn Error>>;
pub type SendableErrorResult<T> = StdResult<T, Box<dyn Error + Send>>;

#[macro_export]
macro_rules! ret_on_err {
  ($result: expr) => {
    match $result {
      Err(err) => return Err(Box::new(err)),
      Ok(v) => v,
    }
  };
}

#[macro_export]
macro_rules! rpc_ret_on_err {
  ($status_code: expr, $result: expr) => {
    match $result {
      Err(err) => {
        return Err(::tonic::Status::new($status_code, format!("{}", err)))
      }
      Ok(v) => v,
    }
  };
}
