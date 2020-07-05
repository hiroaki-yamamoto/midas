use ::url::{Url, ParseError};
use ::std::error::Error;
use ::std::result::Result as StdResult;
use ::tonic::Status;

pub type Result<T> = StdResult<T, Status>;
pub type ParseURLResult = StdResult<Url, ParseError>;
pub type GenericResult<T> = StdResult<T, Box<dyn Error>>;
