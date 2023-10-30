mod base_impl;
mod normal_impl;

use ::std::marker::PhantomData;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

#[derive(Debug)]
pub struct KVSBuilder<'a, Value, LockFnRetValue>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  LockFnRetValue: Send,
{
  channel_name: &'a str,
  _r: PhantomData<Value>,
  _l: PhantomData<LockFnRetValue>,
}

impl<'a, Value, LockFnRetValue> KVSBuilder<'a, Value, LockFnRetValue>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  LockFnRetValue: Send,
{
  pub const fn new(channel_name: &'a str) -> Self {
    return Self {
      channel_name,
      _r: PhantomData,
      _l: PhantomData,
    };
  }
  pub fn build<CMD>(&self, connection: CMD) -> KVS<CMD, Value, LockFnRetValue>
  where
    CMD: Commands + Clone + Send + Sync,
  {
    return KVS::new(connection, self.channel_name.to_string());
  }
}

/// Wrap this struct with Arc if Clone is needed.
#[derive(Debug)]
pub struct KVS<CMD, Value, LockFnRetValue>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
  LockFnRetValue: Send,
{
  pub connection: CMD,
  channel_name: String,
  _r: PhantomData<Value>,
  _l: PhantomData<LockFnRetValue>,
}

impl<CMD, Value, LockFnRetValue> KVS<CMD, Value, LockFnRetValue>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
  LockFnRetValue: Send,
{
  pub(self) fn new(connection: CMD, channel_name: String) -> Self {
    return Self {
      connection,
      channel_name,
      _r: PhantomData,
      _l: PhantomData,
    };
  }
}
