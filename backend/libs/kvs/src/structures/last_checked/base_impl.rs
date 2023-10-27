use ::std::sync::Arc;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::base::{
  Base, ChannelName, Exist, Expiration, Get, ListOp, Lock, Remove, Set, SetOp,
};

impl<R, T> Base for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
  type Commands = T;
  fn __commands__(&self) -> T {
    return self.connection.clone();
  }
}

impl<R, T> ChannelName for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
  fn __channel_name__(&self, key: Arc<String>) -> Arc<String> where {
    return format!("{}:{}", self.channel_name, key).into();
  }
}

impl<R, T> Exist for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
}

impl<R, T> Expiration for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
}

impl<R, T> Get for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
  type Value = R;
}

impl<R, T> ListOp for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
  type Value = R;
}

impl<R, T> Lock for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
}

impl<R, T> Remove for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
}

impl<R, T> Set for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
  type Value = R;
}

impl<R, T> SetOp for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone,
{
  type Value = R;
}
