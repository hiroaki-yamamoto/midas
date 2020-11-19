use ::serde::{Deserialize, Serialize};
use ::std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodParseError {
  method: String,
}

impl MethodParseError {
  pub fn new(method: String) -> Self {
    return Self { method };
  }
}

impl Display for MethodParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    return write!(f, "Unsupported Method: {}", self.method);
  }
}
