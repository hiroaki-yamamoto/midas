mod base_impl;
mod last_checked_impl;

use ::std::marker::PhantomData;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

pub struct KVSBuilder<'a, R>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
{
  channel_name: &'a str,
  _r: PhantomData<R>,
}

impl<'a, R> KVSBuilder<'a, R>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
{
  pub fn new(channel_name: &'a str) -> Self {
    return Self {
      channel_name,
      _r: PhantomData,
    };
  }
  pub fn build<T>(&self, connection: T) -> KVS<R, T>
  where
    T: Commands + Clone,
  {
    return KVS::new(connection, self.channel_name.to_string());
  }
}

/// Wrap this struct with Arc if Clone is needed.
pub struct KVS<V, T>
where
  V: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
  pub connection: T,
  channel_name: String,
  _r: PhantomData<V>,
}

impl<V, T> KVS<V, T>
where
  V: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
  pub(self) fn new(connection: T, channel_name: String) -> Self {
    return Self {
      connection,
      channel_name,
      _r: PhantomData,
    };
  }
}
