use ::http::{status::InvalidStatusCode, StatusCode};
use ::warp::reject::Reject;

use super::status::Status;

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
