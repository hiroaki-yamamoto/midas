use ::std::sync::Arc;

use ::async_trait::async_trait;

use crate::redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use crate::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait SetOp<T, V>: Base<T> + ChannelName
where
  T: AsyncCommands,
  for<'a> V: ToRedisArgs + FromRedisValue + Send + Sync + 'a,
{
  async fn __sadd__(&self, key: Arc<String>, value: V) -> KVSResult<usize> {
    let mut cmd = self.__commands__();
    let channel_name = self.__channel_name__(key);
    return Ok(cmd.sadd(channel_name.as_ref(), value).await?);
  }
  async fn __srem__(&self, key: Arc<String>, value: V) -> KVSResult<usize> {
    let mut cmd = self.__commands__();
    let channel_name = self.__channel_name__(key);
    return Ok(cmd.srem(channel_name.as_ref(), value).await?);
  }
  async fn __smembers__(&self, key: Arc<String>) -> KVSResult<Vec<V>> {
    let mut cmd = self.__commands__();
    let channel_name = self.__channel_name__(key);
    let values: Vec<V> =
      cmd.smembers::<_, Vec<V>>(channel_name.as_ref()).await?;
    return Ok(values);
  }
}
