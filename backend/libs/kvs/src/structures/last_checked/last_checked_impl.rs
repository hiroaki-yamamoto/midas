use ::std::fmt::Debug;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::last_checked::{
  Base, Expiration, FindBefore, Get, ListOp, Remove, Set, SetOp,
};

impl<CMD, Value> Base for KVS<CMD, Value>
where
  CMD: Commands + Clone + Debug + Send + Sync,
  Value: FromRedisValue + ToRedisArgs + Debug + Send + Sync,
{
}

impl<CMD, Value> Expiration for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
{
}

impl<CMD, Value> Get for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
{
}

impl<CMD, Value> ListOp for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
{
}

impl<CMD, Value> Remove for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
{
}

impl<CMD, Value> Set for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
{
}

impl<CMD, Value> FindBefore for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
{
}

impl<CMD, Value> SetOp for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Debug + Send + Sync,
  CMD: Commands + Clone + Debug + Send + Sync,
{
}
