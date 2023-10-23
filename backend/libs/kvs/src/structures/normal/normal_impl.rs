use ::std::future::Future;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::normal::{
  Exist, Expiration, Get, ListOp, Lock, Remove, Set,
};

impl<R, T, Ft, Fr> Exist<T> for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Expiration<T> for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Get<T, R> for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> ListOp<T, R> for KVS<R, T, Ft, Fr>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Lock<T, Ft, Fr> for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Remove<T> for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Set<T, R> for KVS<R, T, Ft, Fr>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}
