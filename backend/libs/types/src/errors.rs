use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FormatResult};

use ::serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct VecElementErr<T>
where
  T: Error,
{
  pub index: usize,
  pub err: T,
}

impl<T> Display for VecElementErr<T>
where
  T: Error,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "{:?}", self);
  }
}

impl<T> Error for VecElementErr<T> where T: Error {}

impl<T> VecElementErr<T>
where
  T: Error,
{
  pub fn new(index: usize, err: T) -> Self {
    return Self { index, err };
  }
}
