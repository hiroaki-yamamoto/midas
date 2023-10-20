use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::tokio::sync::Mutex;

use crate::redis::AsyncCommands as Commands;
use crate::redis::FromRedisValue;
use crate::traits::normal::{
  Base, ChannelName, Exist, Expiration, Get, ListOp, Lock, Remove, Set,
};

pub struct KVSBuilder<R>
where
  R: FromRedisValue,
{
  channel_name: String,
  _r: PhantomData<R>,
}

impl<R> KVSBuilder<R>
where
  R: FromRedisValue,
{
  pub fn new(channel_name: String) -> Self {
    return Self {
      channel_name,
      _r: PhantomData::default(),
    };
  }
  pub fn build<T>(&self, connection: Arc<Mutex<T>>) -> Arc<Mutex<KVS<R, T>>>
  where
    T: Commands,
  {
    return KVS::new(connection, self.channel_name).into();
  }
}

/// Wrap this struct with Arc if Clone is needed.
pub struct KVS<R, T>
where
  R: FromRedisValue,
  T: Commands,
{
  pub connection: Arc<Mutex<T>>,
  channel_name: String,
  _r: PhantomData<R>,
}

impl<R, T> KVS<R, T>
where
  R: FromRedisValue,
  T: Commands,
{
  pub fn new(connection: Arc<Mutex<T>>, channel_name: String) -> Self {
    return Self {
      connection,
      channel_name,
      _r: PhantomData::default(),
    };
  }
}

impl<R, T> Base<T> for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands,
{
  fn commands(&self) -> Arc<Mutex<T>> {
    return self.connection.clone();
  }
}

impl<R, T> ChannelName for KVS<R, T>
where
  R: FromRedisValue,
  T: Commands,
{
  fn channel_name(&self, key: &str) -> String where {
    return format!("{}.{}", self.channel_name, key);
  }
}
