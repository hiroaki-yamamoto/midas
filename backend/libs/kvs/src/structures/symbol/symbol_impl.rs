use ::std::sync::Arc;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::symbol::{ChannelName, Get, Remove, Set};

impl<R, T> ChannelName for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
  fn channel_name(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
  ) -> Arc<String> {
    let channel_name = format!("{}:{}:{}", self.channel_name, exchange, symbol);
    return channel_name.into();
  }
}

impl<R, T> Get for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone + Sync,
{
}

impl<R, T> Remove for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone + Sync,
{
}

impl<R, T> Set for KVS<R, T>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone + Sync,
{
}
