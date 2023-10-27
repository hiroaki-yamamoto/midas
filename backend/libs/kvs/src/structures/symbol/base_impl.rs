use crate::redis::AsyncCommands as Commands;
use crate::redis::FromRedisValue;

use super::KVS;
use crate::traits::base::Base;

impl<R, T> Base for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
  fn __commands__(&self) -> T {
    return self.connection.clone();
  }
}
