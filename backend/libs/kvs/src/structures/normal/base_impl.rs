use ::std::future::Future;
use ::std::sync::Arc;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::base::{
  Base, ChannelName, Exist, Expiration, Get, ListOp, Lock, Remove, Set,
};

impl<R, T, Ft, Fr> Base<T> for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
  fn __commands__(&self) -> T {
    return self.connection.clone();
  }
}

impl<R, T, Ft, Fr> ChannelName for KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
  fn __channel_name__(&self, key: Arc<String>) -> Arc<String> where {
    return format!("{}:{}", self.channel_name, key).into();
  }
}

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
