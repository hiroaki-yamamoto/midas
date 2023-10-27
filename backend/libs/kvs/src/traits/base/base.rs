use ::std::sync::Arc;

use ::redis::AsyncCommands as Commands;

pub trait ClonnableCommands: Commands + Clone + Send + Sync {}

pub trait Base {
  fn __commands__(&self) -> Arc<dyn ClonnableCommands>;
}
