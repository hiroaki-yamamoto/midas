use ::std::future::Future;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::normal::{
  Exist, Expiration, Get, ListOp, Lock, Remove, Set,
};

impl<R, T, Ft, Fr> Exist for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Expiration for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Get for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> ListOp for KVS<R, T, Ft, Fr>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Lock for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Remove for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Set for KVS<R, T, Ft, Fr>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}
