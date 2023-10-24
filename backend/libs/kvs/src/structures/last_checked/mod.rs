mod base_impl;
mod last_checked_impl;

use ::std::future::Future;
use ::std::marker::PhantomData;

use redis::ToRedisArgs;

use crate::redis::AsyncCommands as Commands;
use crate::redis::FromRedisValue;

use crate::traits::last_checked::LastCheckedKVS;

pub struct KVSBuilder<'a, R>
where
  R: FromRedisValue,
{
  channel_name: &'a str,
  _r: PhantomData<R>,
}

impl<'a, R> KVSBuilder<'a, R>
where
  R: FromRedisValue,
{
  pub fn new(channel_name: &'a str) -> Self {
    return Self {
      channel_name,
      _r: PhantomData,
    };
  }
  pub fn build<T, Ft, Fr>(&self, connection: T) -> KVS<R, T, Ft, Fr>
  where
    T: Commands + Clone,
    Ft: Future<Output = Fr> + Send,
    Fr: Send,
  {
    return KVS::new(connection, self.channel_name.to_string());
  }
}

/// Wrap this struct with Arc if Clone is needed.
pub struct KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
  pub connection: T,
  channel_name: String,
  _r: PhantomData<R>,
  _ft: PhantomData<Ft>,
  _fr: PhantomData<Fr>,
}

impl<R, T, Ft, Fr> KVS<R, T, Ft, Fr>
where
  R: FromRedisValue,
  T: Commands + Clone,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
  pub(self) fn new(connection: T, channel_name: String) -> Self {
    return Self {
      connection,
      channel_name,
      _r: PhantomData,
      _fr: PhantomData,
      _ft: PhantomData,
    };
  }
}
