use ::chrono::{NaiveDateTime, Utc};
use ::serde_json::Value;

use super::errors::ParseError;
use super::{DateTime, ThreadSafeResult};

pub fn cast_datetime(
  fld_name: &str,
  value: &Value,
) -> ThreadSafeResult<DateTime> {
  return match value.as_i64() {
    Some(n) => Ok(cast_datetime_from_i64(n)),
    None => Err(Box::new(ParseError::new(fld_name))),
  };
}

pub fn cast_datetime_from_i64(value: i64) -> DateTime {
  let (epoch, mils) = (value / 1000, value % 1000);
  return DateTime::from_utc(
    NaiveDateTime::from_timestamp(epoch, (mils * 1000).abs() as u32),
    Utc,
  );
}

pub fn cast_f64(fld_name: &str, value: &Value) -> ThreadSafeResult<f64> {
  return match value.as_str() {
    Some(s) => Ok(s.parse()?),
    None => return Err(Box::new(ParseError::new(fld_name))),
  };
}

pub fn cast_i64(fld_name: &str, value: &Value) -> ThreadSafeResult<i64> {
  return match value.as_i64() {
    Some(n) => Ok(n),
    None => return Err(Box::new(ParseError::new(fld_name))),
  };
}
