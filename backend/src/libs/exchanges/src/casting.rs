use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FormatResult};
use ::chrono::{DateTime, NaiveDateTime, Utc};
use ::serde_json::Value;

type CastResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub(crate) struct ParseError {
  fld_name: String
}

impl ParseError {
  fn new(fld_name: &str) -> Self {
    return ParseError{fld_name: fld_name.into()};
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
  value: Value,
) -> CastResult<DateTime<Utc>> {
  let epoch = match value.as_i64() {
    Some(n) => n,
    None => return Err(Box::new(ParseError::new(fld_name))),
  };
  return Ok(DateTime::from_utc(
    NaiveDateTime::from_timestamp(epoch, 0), Utc,
  ));
}

pub(crate) fn cast_f64(fld_name: &str, value: Value) -> CastResult<f64> {
  return match value.as_str() {
    Some(s) => Ok(s.parse()?),
    None => return Err(Box::new(ParseError::new(fld_name))),
  };
}

pub(crate) fn cast_i64(fld_name: &str, value: Value) -> CastResult<i64> {
  return match value.as_i64() {
    Some(n) => Ok(n),
    None => return Err(Box::new(ParseError::new(fld_name))),
  };
}
