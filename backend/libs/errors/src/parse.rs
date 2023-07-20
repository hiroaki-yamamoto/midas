use ::std::fmt::Debug;

use ::err_derive::Error;

#[derive(Debug, Default, Clone, Error)]
#[error(
  display = "Failed to parse: (field: {:?}, input: {:?}, desc: {:?})",
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
  pub fn new<S, T, U>(
    field: Option<S>,
    input: Option<T>,
    desc: Option<U>,
  ) -> Self
  where
    S: AsRef<str>,
    T: AsRef<str>,
    U: AsRef<str>,
  {
    return Self {
      field: field.map(|s| s.as_ref().to_string()),
      input: input.map(|s| s.as_ref().to_string()),
      desc: desc.map(|s| s.as_ref().into()),
    };
  }
}

pub type ParseResult<T> = Result<T, ParseError>;
