use ::std::sync::{Arc, Mutex};
use std::ops::{Deref, DerefMut};

pub use ::redis::Commands as RedisCommands;

#[derive(Clone)]
pub struct Connection<T>
where
  T: RedisCommands,
{
  pub connection: Arc<Mutex<T>>,
}

impl<T> From<T> for Connection<T>
where
  T: RedisCommands,
{
  fn from(value: T) -> Self {
    return Self {
      connection: Arc::new(Mutex::new(value)),
    };
  }
}

impl<T> From<Arc<Mutex<T>>> for Connection<T>
where
  T: RedisCommands,
{
  fn from(value: Arc<Mutex<T>>) -> Self {
    return Self { connection: value };
  }
}

impl<T> Deref for Connection<T>
where
  T: RedisCommands,
{
  type Target = Arc<Mutex<T>>;

  fn deref(&self) -> &Self::Target {
    &self.connection
  }
}

impl<T> DerefMut for Connection<T>
where
  T: RedisCommands,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.connection
  }
}
