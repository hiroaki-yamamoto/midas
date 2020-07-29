use ::std::error::Error;
use ::std::result::Result as StdResult;
use ::tonic::Status;
use ::url::{ParseError, Url};

pub type Result<T> = StdResult<T, Status>;
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
