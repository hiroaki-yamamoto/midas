use ::std::error::Error as ErrTrait;

use ::thiserror::Error;

use ::serde::Serialize;

#[derive(Debug, Clone, Serialize, Error)]
#[error("Error (index: {}, err: {:?}", index, err)]
pub struct VecElementErr<T>
where
  T: ErrTrait,
{
  pub index: usize,
  pub err: T,
}

impl<T> VecElementErr<T>
where
  T: ErrTrait,
{
  pub fn new(index: usize, err: T) -> Self {
    return Self { index, err };
  }
}

pub type RawVecElemErrs<T> = Vec<VecElementErr<T>>;

#[derive(Debug, Clone, Serialize, Error)]
#[error("Multiple Errors: {:?}", errors)]
pub struct VecElementErrs<T>
where
  T: ErrTrait,
{
  pub errors: Vec<VecElementErr<T>>,
}

impl<T> From<RawVecElemErrs<T>> for VecElementErrs<T>
where
  T: ErrTrait,
{
  fn from(v: Vec<VecElementErr<T>>) -> Self {
    return Self { errors: v };
  }
}
