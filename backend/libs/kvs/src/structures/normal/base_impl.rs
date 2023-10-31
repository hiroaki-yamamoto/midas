use ::std::fmt::Debug;
use ::std::sync::Arc;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::base::{
  Base, ChannelName, Exist, Expiration, Get, ListOp, Lock, OptExecution,
  Remove, Set,
};

impl<CMD, Value, LockFnRetValue> Base for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
  LockFnRetValue: Send + Debug,
{
  type Commands = CMD;
  fn __commands__(&self) -> CMD {
    return self.connection.clone();
  }
}

impl<CMD, Value, LockFnRetValue> ChannelName for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
  LockFnRetValue: Send + Debug,
{
  fn __channel_name__(&self, key: Arc<String>) -> Arc<String> where {
    return format!("{}:{}", self.channel_name, key).into();
  }
}

impl<CMD, Value, LockFnRetValue> Exist for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
  LockFnRetValue: Send + Debug,
{
}

impl<CMD, Value, LockFnRetValue> Expiration for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
  LockFnRetValue: Send + Debug,
{
}

impl<CMD, Value, LockFnRetValue> Get for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
  LockFnRetValue: Send + Debug,
{
  type Value = Value;
}

impl<CMD, Value, LockFnRetValue> ListOp for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
  LockFnRetValue: Send + Debug,
{
  type Value = Value;
}

impl<CMD, Value, LockFnRetValue> Lock for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
  LockFnRetValue: Send + Debug,
{
  type Value = LockFnRetValue;
}

impl<CMD, Value, LockFnRetValue> OptExecution
  for KVS<CMD, Value, LockFnRetValue>
where
  Value: FromRedisValue + ToRedisArgs + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
  LockFnRetValue: Send + Debug,
{
}

impl<CMD, Value, LockFnRetValue> Remove for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
  LockFnRetValue: Send + Debug,
{
}

impl<CMD, Value, LockFnRetValue> Set for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
  LockFnRetValue: Send + Debug,
{
  type Value = Value;
}
