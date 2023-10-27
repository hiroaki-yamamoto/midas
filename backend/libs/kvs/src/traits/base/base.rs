use crate::redis::aio::MultiplexedConnection;

pub trait Base {
  fn __commands__(&self) -> MultiplexedConnection;
}
