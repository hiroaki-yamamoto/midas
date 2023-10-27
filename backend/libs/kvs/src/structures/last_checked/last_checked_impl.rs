use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::last_checked::{
  Base, Expiration, FindBefore, Get, ListOp, Remove, Set, SetOp,
};

impl<R, T> Base for KVS<R, T>
where
  T: Commands + Clone + Send,
  R: FromRedisValue + ToRedisArgs + Send + Sync,
{
}

impl<R, T> Expiration for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone + Send + Sync,
{
}

impl<R, T> Get for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone + Send,
{
}

impl<R, T> ListOp for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone + Send,
{
}

impl<R, T> Remove for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone + Send,
{
}

impl<R, T> Set for KVS<R, T>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone + Send,
{
}

impl<R, T> FindBefore for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone + Send,
{
}

impl<R, T> SetOp for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send + Sync,
  T: Commands + Clone + Send,
{
}
