use ::serde::{Deserialize, Serialize};
use ::warp::http::{status::InvalidStatusCode, StatusCode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
  code: u16,
  message: String,
}

impl Status {
  pub fn new<T>(code: StatusCode, msg: T) -> Self
  where
    T: AsRef<str>,
  {
    return Self {
      code: code.as_u16(),
      message: msg.as_ref().to_string(),
    };
  }
  pub fn new_int(code: u16, msg: &str) -> Self {
    return Self {
      code,
      message: String::from(msg),
    };
  }
}

impl ::std::convert::TryFrom<Status> for StatusCode {
  type Error = InvalidStatusCode;
  fn try_from(value: Status) -> Result<Self, Self::Error> {
    return Self::from_u16(value.code);
  }
}
