use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::normal::{
  Exist, Expiration, Get, ListOp, Lock, Remove, Set,
};

impl<CMD, Value> Exist for KVS<CMD, Value>
where
  Value: ToRedisArgs + FromRedisValue + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> Expiration for KVS<CMD, Value>
where
  Value: ToRedisArgs + FromRedisValue + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> Get for KVS<CMD, Value>
where
  Value: ToRedisArgs + FromRedisValue + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> ListOp for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> Lock for KVS<CMD, Value>
where
  Value: ToRedisArgs + FromRedisValue + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> Remove for KVS<CMD, Value>
where
  Value: ToRedisArgs + FromRedisValue + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}

impl<CMD, Value> Set for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync,
  CMD: Commands + Clone + Send + Sync,
{
}
