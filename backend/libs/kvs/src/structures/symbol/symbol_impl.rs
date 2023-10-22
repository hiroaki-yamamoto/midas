use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::symbol::{ChannelName, Get, Remove, Set};

impl<R, T> ChannelName for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
  fn channel_name(&self, exchange: &str, symbol: &str) -> String {
    let channel_name = format!("{}:{}:{}", self.channel_name, exchange, symbol);
    return channel_name;
  }
}

impl<R, T> Get<T, R> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone + Sync,
{
}

impl<R, T> Remove<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone + Sync,
{
}

impl<R, T> Set<T, R> for KVS<R, T>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone + Sync,
{
}
