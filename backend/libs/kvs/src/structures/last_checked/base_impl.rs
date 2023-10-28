use ::std::sync::Arc;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::base::{
  Base, ChannelName, Exist, Expiration, Get, ListOp, Lock, OptExecution,
  Remove, Set, SetOp,
};

impl<CMD, Value> Base for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
  type Commands = CMD;
  fn __commands__(&self) -> CMD {
    return self.connection.clone();
  }
}

impl<CMD, Value> ChannelName for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
  fn __channel_name__(&self, key: Arc<String>) -> Arc<String> where {
    return format!("{}:{}", self.channel_name, key).into();
  }
}

impl<CMD, Value> Exist for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> Expiration for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> Get for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
  type Value = Value;
}

impl<CMD, Value> ListOp for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
  type Value = Value;
}

impl<CMD, Value> Lock for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> OptExecution for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> Remove for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> Set for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
  type Value = Value;
}

impl<CMD, Value> SetOp for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
  type Value = Value;
}
