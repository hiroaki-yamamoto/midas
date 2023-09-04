use ::std::fmt::Display;

use ::tokio::sync::Mutex;

use ::redis::{Commands, FromRedisValue, ToRedisArgs};

pub trait Base<T, V>
where
  T: Commands,
  V: FromRedisValue + ToRedisArgs,
{
  fn commands(&self) -> Mutex<T>;
}
