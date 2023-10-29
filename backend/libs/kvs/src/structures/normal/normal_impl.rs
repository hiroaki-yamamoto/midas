use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::normal::{
  Exist, Expiration, Get, ListOp, Lock, Remove, Set,
};

impl<CMD, Value, LockFnRetValue> Exist for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
  LockFnRetValue: Send,
{
}

impl<CMD, Value, LockFnRetValue> Expiration for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
  LockFnRetValue: Send,
{
}

impl<CMD, Value, LockFnRetValue> Get for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
  LockFnRetValue: Send,
{
}

impl<CMD, Value, LockFnRetValue> ListOp for KVS<CMD, Value, LockFnRetValue>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
  LockFnRetValue: Send,
{
}

impl<CMD, Value, LockFnRetValue> Lock for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
  LockFnRetValue: Send,
{
}

impl<CMD, Value, LockFnRetValue> Remove for KVS<CMD, Value, LockFnRetValue>
where
  Value: ToRedisArgs + FromRedisValue + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
  LockFnRetValue: Send,
{
}

impl<CMD, Value, LockFnRetValue> Set for KVS<CMD, Value, LockFnRetValue>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
  LockFnRetValue: Send,
{
}
