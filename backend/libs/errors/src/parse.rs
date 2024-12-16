use ::std::error::Error;
use ::std::fmt::Debug;

use ::thiserror::Error as ErrorDerive;

#[derive(Debug, Default, Clone, ErrorDerive)]
#[error(
  "Failed to parse: (field: {:?}, input: {:?}, desc: {:?})",
  field,
  input,
  desc
)]
pub struct ParseError {
  pub field: Option<String>,
  pub input: Option<String>,
  pub desc: Option<String>,
}

impl ParseError {
  pub fn new(
    field: Option<&str>,
    input: Option<&str>,
    desc: Option<&str>,
  ) -> Self {
    return Self {
      field: field.map(|s| s.into()),
      input: input.map(|s| s.into()),
      desc: desc.map(|s| s.into()),
    };
  }
  pub fn raise_parse_err<'a, U>(
    field: &'a str,
    input: &'a str,
  ) -> impl Fn(U) -> ParseError + 'a
  where
    U: Error,
  {
    return move |err: U| {
      return ParseError::new(
        Some(field.into()),
        Some(input.into()),
        Some(&err.to_string()),
      );
    };
  }
}

pub type ParseResult<T> = Result<T, ParseError>;
