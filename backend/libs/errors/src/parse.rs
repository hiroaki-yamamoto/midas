use ::std::error::Error;
use ::std::fmt::{Debug, Display, Formatter, Result as FormatResult};

#[derive(Debug)]
pub struct ParseError {
  raw_input: String,
}

impl ParseError {
  pub fn new(raw_input: String) -> Self {
    return Self { raw_input };
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Invalid: {}", self.raw_input);
  }
}

impl Error for ParseError {}
