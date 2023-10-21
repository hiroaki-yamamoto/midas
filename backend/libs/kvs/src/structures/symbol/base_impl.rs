use ::std::sync::Arc;

use ::tokio::sync::Mutex;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::base::Base;

impl<R, T> Base<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands,
{
  fn commands(&self) -> Arc<Mutex<T>> {
    return self.connection.clone();
  }
}
