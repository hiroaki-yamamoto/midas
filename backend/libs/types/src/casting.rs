use ::mongodb::bson::DateTime;
use ::rug::Float;
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

pub fn cast_f(fld_name: &str, value: &Value) -> ParseResult<Float> {
  return value
    .as_str()
    .ok_or(ParseError::new(
      Some(fld_name),
      Some(value.to_string()),
      None::<&str>,
    ))
    .and_then(|txt| {
      let parsed = Float::parse(txt).map_err(|e| {
        return ParseError::new(
          Some(fld_name),
          Some(value.to_string()),
          e.to_string().into(),
        );
      })?;
      return Ok(Float::with_val(32, parsed));
    });
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
