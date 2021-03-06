use super::entities::{Exchanges, InsertOneResult, Status};
use ::bson::oid::ObjectId;
use ::http::{status::InvalidStatusCode, StatusCode};
use ::warp::reject::Reject;

impl Exchanges {
  pub fn as_string(&self) -> String {
    return String::from(match self {
      Exchanges::Unknown => "Unknown",
      Exchanges::Binance => "binance",
    });
  }
}

impl ::std::str::FromStr for Exchanges {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let ret: Self = match s.to_lowercase().as_str() {
      "binance" => Exchanges::Binance,
      _ => return Err(format!("Failed to parse the exchange argument: {}", s)),
    };
    return Ok(ret);
  }
}

impl From<Exchanges> for String {
  fn from(exchange: Exchanges) -> Self {
    return exchange.as_string();
  }
}

impl Status {
  pub fn new<T>(code: StatusCode, msg: T) -> Self
  where
    T: AsRef<str>,
  {
    return Self {
      code: code.as_u16() as u32,
      message: msg.as_ref().to_string(),
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
