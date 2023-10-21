mod base_impl;
mod normal_impl;

use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::tokio::sync::Mutex;

use crate::redis::AsyncCommands as Commands;
use crate::redis::FromRedisValue;

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
    return Arc::new(KVS::new(connection, self.channel_name.clone()).into());
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
