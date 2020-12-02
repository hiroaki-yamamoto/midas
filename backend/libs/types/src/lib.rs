use ::std::error::Error;
use ::std::result::Result as StdResult;

use ::chrono::{DateTime as ChronoDateTime, Utc};
use ::url::{ParseError, Url};

pub type ParseURLResult = StdResult<Url, ParseError>;
pub type GenericResult<T> = StdResult<T, Box<dyn Error>>;
pub type SendableErrorResult<T> = StdResult<T, Box<dyn Error + Send>>;
pub type DateTime = ChronoDateTime<Utc>;

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
macro_rules! reply_on_err {
  ($code: expr, $result: expr) => {
    match $result {
      Err(err) => {
        let resp: Box<dyn ::warp::Reply> =
          Box::new(::warp::reply::with_status(
            ::warp::reply::json(&::types::Status::new(
              $code,
              format!("{}", err).as_str(),
            )),
            $code,
          ));
        return resp;
      }
      Ok(v) => v,
    }
  };
}
