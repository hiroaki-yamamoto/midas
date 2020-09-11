use ::serde::{Deserialize, Serialize};
use ::tonic::{Code, Status as TonicStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
  code: i32,
  message: String,
}

impl Status {
  pub fn new(code: Code, msg: &str) -> Self {
    return Self {
      code: code as i32,
      message: String::from(msg),
    };
  }
  pub fn from_tonic_status(st: &TonicStatus) -> Self {
    return Self {
      code: st.code() as i32,
      message: String::from(st.message()),
    };
  }
  pub fn code(&self) -> Code {
    return self.code.into();
  }
}
