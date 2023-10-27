use ::std::future::Future;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::last_checked::{
  Base, Expiration, FindBefore, Get, ListOp, Remove, Set, SetOp,
};

impl<R, T, Ft, Fr> Base for KVS<R, T, Ft, Fr>
where
  T: Commands + Clone + Send,
  R: FromRedisValue,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Expiration for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone + Send,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Get for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue + ToRedisArgs + Send,
  T: Commands + Clone + Send,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> ListOp for KVS<R, T, Ft, Fr>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone + Send,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Remove for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone + Send,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> Set for KVS<R, T, Ft, Fr>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone + Send,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> FindBefore for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone + Send,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}

impl<R, T, Ft, Fr> SetOp for KVS<R, T, Ft, Fr>
where
  for<'a> R: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
  T: Commands + Clone + Send,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
}
