use ::std::sync::Arc;
use ::tokio::sync::Mutex;

use ::redis::Commands;

pub trait Base<T>
where
  T: Commands,
{
  fn commands(&self) -> Arc<Mutex<T>>;
}
