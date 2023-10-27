use crate::redis::AsyncCommands;

pub trait Base {
  type Commands: AsyncCommands + Send + Sync;

  fn __commands__(&self) -> Self::Commands;
}
