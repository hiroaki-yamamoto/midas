use ::tokio::sync::Mutex;

use ::redis::Commands;

pub trait Base<T>
where
  T: Commands,
{
  fn commands(&self) -> Mutex<T>;
}
