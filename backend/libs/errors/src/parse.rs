use ::std::fmt::Debug;

use ::err_derive::Error;

#[derive(Debug, Default, Clone, Error)]
#[error(display = "Failed to parse: (field: {:?}, input: {:?})", field, input)]
pub struct ParseError {
  pub field: Option<String>,
  pub input: Option<String>,
}

impl ParseError {
  pub fn new<S, T>(field: Option<S>, input: Option<T>) -> Self
  where
    S: AsRef<str>,
    T: AsRef<str>,
  {
    return Self {
      field: field.map(|s| s.as_ref().to_string()),
      input: input.map(|s| s.as_ref().to_string()),
    };
  }
}
