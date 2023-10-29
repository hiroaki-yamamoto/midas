mod base_impl;
mod last_checked_impl;

use ::std::marker::PhantomData;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

pub struct KVSBuilder<'a, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
{
  channel_name: &'a str,
  _r: PhantomData<Value>,
}

impl<'a, Value> KVSBuilder<'a, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
{
  pub const fn new(channel_name: &'a str) -> Self {
    return Self {
      channel_name,
      _r: PhantomData,
    };
  }
  pub fn build<CMD>(&self, connection: CMD) -> KVS<CMD, Value>
  where
    CMD: Commands + Clone + Send + Sync,
  {
    return KVS::new(connection, self.channel_name.to_string());
  }
}

/// Wrap this struct with Arc if Clone is needed.
pub struct KVS<CMD, Value>
where
  CMD: Commands + Clone + Send + Sync,
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
{
  pub connection: CMD,
  channel_name: String,
  _r: PhantomData<Value>,
}

impl<CMD, Value> KVS<CMD, Value>
where
  CMD: Commands + Clone + Send + Sync,
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
{
  pub(self) fn new(connection: CMD, channel_name: String) -> Self {
    return Self {
      connection,
      channel_name,
      _r: PhantomData,
    };
  }
}
