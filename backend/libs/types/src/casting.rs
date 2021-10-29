use ::mongodb::bson::DateTime;
use ::serde_json::Value;

use super::StdResult;
use ::errors::ParseError;

pub fn cast_datetime(
  fld_name: &str,
  value: &Value,
) -> StdResult<DateTime, ParseError> {
  return match value.as_i64() {
    Some(n) => Ok(cast_datetime_from_i64(n)),
    None => Err(ParseError::new(Some(fld_name), Some(value.to_string()))),
  };
}

pub fn cast_datetime_from_i64(value: i64) -> DateTime {
  return DateTime::from_millis(value);
}

pub fn cast_f64(fld_name: &str, value: &Value) -> StdResult<f64, ParseError> {
  let err = ParseError::new(Some(fld_name), Some(value.to_string()));
  return match value.as_str() {
    Some(s) => Ok(s.parse().map_err(|_| err))?,
    None => return Err(err),
  };
}

pub fn cast_i64(fld_name: &str, value: &Value) -> StdResult<i64, ParseError> {
  return match value.as_i64() {
    Some(n) => Ok(n),
    None => {
      return Err(ParseError::new(Some(fld_name), Some(value.to_string())))
    }
  };
}
