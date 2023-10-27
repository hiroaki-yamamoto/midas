use crate::redis::AsyncCommands;

pub trait Base {
  type Commands: AsyncCommands;

  fn __commands__(&self) -> Self::Commands;
}
