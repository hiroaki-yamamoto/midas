use ::mongodb::bson::DateTime;
use ::serde_json::Value;

use ::errors::{ParseError, ParseResult};

pub fn cast_datetime(fld_name: &str, value: &Value) -> ParseResult<DateTime> {
  return match value.as_i64() {
    Some(n) => Ok(cast_datetime_from_i64(n)),
    None => Err(ParseError::new(
      Some(fld_name),
      Some(value.to_string()),
      None::<&str>,
    )),
  };
}

pub fn cast_datetime_from_i64(value: i64) -> DateTime {
  return DateTime::from_millis(value);
}

pub fn cast_f64(fld_name: &str, value: &Value) -> ParseResult<f64> {
  let err =
    ParseError::new(Some(fld_name), Some(value.to_string()), None::<&str>);
  return match value.as_str() {
    Some(s) => Ok(s.parse().map_err(|_| err))?,
    None => return Err(err),
  };
}

pub fn cast_i64(fld_name: &str, value: &Value) -> ParseResult<i64> {
  return match value.as_i64() {
    Some(n) => Ok(n),
    None => {
      return Err(ParseError::new(
        Some(fld_name),
        Some(value.to_string()),
        None::<&str>,
      ))
    }
  };
}
