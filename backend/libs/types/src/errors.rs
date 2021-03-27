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

pub type RawVecElemErrs<T> = Vec<VecElementErr<T>>;

#[derive(Debug, Clone, Serialize)]
pub struct VecElementErrs<T>
where
  T: Error,
{
  pub errors: Vec<VecElementErr<T>>,
}

impl<T> Display for VecElementErrs<T>
where
  T: Error,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "{:?}", self);
  }
}

impl<T> Error for VecElementErrs<T> where T: Error {}
impl<T> From<RawVecElemErrs<T>> for VecElementErrs<T>
where
  T: Error,
{
  fn from(v: Vec<VecElementErr<T>>) -> Self {
    return Self { errors: v };
  }
}
