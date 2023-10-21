use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::last_checked::{Base, Expiration, Get, ListOp, Remove, Set};

impl<R, T> Base<T> for KVS<R, T>
where
  T: Commands + Send,
  R: FromRedisValue,
{
}

impl<R, T> Expiration<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands,
{
}

impl<R, T> Get<T, R> for KVS<R, T>
where
  R: FromRedisValue + ToRedisArgs + Send,
  T: Commands + Send,
{
}

impl<R, T> ListOp<T, R> for KVS<R, T>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands,
{
}

impl<R, T> Remove<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands,
{
}

impl<R, T> Set<T, R> for KVS<R, T>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands,
{
}
