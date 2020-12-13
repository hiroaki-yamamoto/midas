use ::chrono::{DateTime, NaiveDateTime, Utc};
use ::serde_json::Value;
use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FormatResult};
use ::std::marker::Send;
use ::types::{ret_on_err, DateTime as UTCDateTime};

type CastResult<T> = Result<T, Box<dyn Error + Send>>;

#[derive(Debug)]
pub(crate) struct ParseError {
  fld_name: String,
}

impl ParseError {
  fn new(fld_name: &str) -> Self {
    return ParseError {
      fld_name: fld_name.into(),
    };
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Failed to parse '{}", self.fld_name);
  }
}

impl Error for ParseError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

pub(crate) fn cast_datetime(
  fld_name: &str,
  value: &Value,
) -> CastResult<DateTime<Utc>> {
  return match value.as_i64() {
    Some(n) => Ok(cast_datetime_from_i64(n)),
    None => Err(Box::new(ParseError::new(fld_name))),
  };
}

pub(crate) fn cast_datetime_from_i64(value: i64) -> UTCDateTime {
  let (epoch, mils) = (value / 1000, value % 1000);
  return UTCDateTime::from_utc(
    NaiveDateTime::from_timestamp(epoch, (mils * 1000).abs() as u32),
    Utc,
  );
}

pub(crate) fn cast_f64(fld_name: &str, value: &Value) -> CastResult<f64> {
  return match value.as_str() {
    Some(s) => Ok(ret_on_err!(s.parse())),
    None => return Err(Box::new(ParseError::new(fld_name))),
  };
}

pub(crate) fn cast_i64(fld_name: &str, value: &Value) -> CastResult<i64> {
  return match value.as_i64() {
    Some(n) => Ok(n),
    None => return Err(Box::new(ParseError::new(fld_name))),
  };
}
