use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::normal::{
  Exist, Expiration, Get, ListOp, Lock, Remove, Set,
};

impl<R, T> Exist<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
}

impl<R, T> Expiration<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
}

impl<R, T> Get<T, R> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
}

impl<R, T> ListOp<T, R> for KVS<R, T>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone,
{
}

impl<R, T> Lock<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
}

impl<R, T> Remove<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands + Clone,
{
}

impl<R, T> Set<T, R> for KVS<R, T>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone,
{
}
