use crate::redis::AsyncCommands;
use ::async_trait::async_trait;

#[async_trait]
pub trait Base {
  type Commands: AsyncCommands + Send + Sync;

  fn __commands__(&self) -> Self::Commands;
}
