use ::std::error::Error;
use ::std::fmt::{Debug, Display, Formatter, Result as FormatResult};

#[derive(Debug, Clone)]
pub struct ObjectNotFound {
  entity: String,
}

impl ObjectNotFound {
  pub fn new(entity: String) -> Self {
    return Self { entity };
  }
}

impl Display for ObjectNotFound {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Entity {} Not Found", self.entity);
  }
}

impl Error for ObjectNotFound {}
