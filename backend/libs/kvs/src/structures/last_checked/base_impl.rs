use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::base::{
  Base, ChannelName, Exist, Expiration, Get, ListOp, Lock, Remove, Set,
};

impl<R, T> Base<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
  fn commands(&self) -> T {
    return self.connection.clone();
  }
}

impl<R, T> ChannelName for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
  fn channel_name(&self, key: &str) -> String where {
    return format!("{}:{}", self.channel_name, key);
  }
}

impl<R, T> Exist<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
}

impl<R, T> Expiration<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
}

impl<R, T> Get<T, R> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
}

impl<R, T> ListOp<T, R> for KVS<R, T>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone,
{
}

impl<R, T> Lock<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
}

impl<R, T> Remove<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
}

impl<R, T> Set<T, R> for KVS<R, T>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone,
{
}
