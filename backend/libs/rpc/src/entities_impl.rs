use ::std::convert::TryFrom;

use super::entities::{Exchanges, InsertOneResult, Status};
use ::bson::oid::ObjectId;
use ::http::{status::InvalidStatusCode, StatusCode};
use ::warp::reject::Reject;
use ::warp::Filter;

use ::errors::ParseError;

impl Exchanges {
  pub fn by_param(
  ) -> impl Filter<Extract = (Exchanges,), Error = ::warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static {
    return ::warp::path::param::<i32>()
      .and_then(|param: i32| async move {
        return Exchanges::try_from(param)
          .map(|exchange| (exchange,))
          .map_err(|_| ::warp::reject::not_found());
      })
      .untuple_one();
  }
}

impl ::std::str::FromStr for Exchanges {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let ret: Self = match s.to_lowercase().as_str() {
      "binance" => Exchanges::Binance,
      _ => return Err(ParseError::new(None::<&str>, Some(s), None::<&str>)),
    };
    return Ok(ret);
  }
}

impl Status {
  pub fn new(code: StatusCode, msg: &str) -> Self {
    return Self {
      code: code.as_u16() as u32,
      message: msg.to_string(),
    };
  }
  pub fn new_int(code: u32, msg: &str) -> Self {
    return Self {
      code,
      message: String::from(msg),
    };
  }
}

impl Reject for Status {}

impl ::std::convert::TryFrom<Status> for StatusCode {
  type Error = InvalidStatusCode;
  fn try_from(value: Status) -> Result<Self, Self::Error> {
    let code = u16::try_from(value.code).unwrap_or(::std::u16::MAX);
    return Self::from_u16(code);
  }
}

impl From<Option<ObjectId>> for InsertOneResult {
  fn from(value: Option<ObjectId>) -> Self {
    return Self {
      id: value.map(|v| v.to_hex()).unwrap_or(String::default()),
    };
  }
}
