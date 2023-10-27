use ::std::sync::Arc;

use ::async_trait::async_trait;

use crate::redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use crate::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait SetOp: Base + ChannelName {
  async fn __sadd__(
    &self,
    key: Arc<String>,
    value: Arc<dyn ToRedisArgs>,
  ) -> KVSResult<usize> {
    let mut cmd = self.__commands__();
    let channel_name = self.__channel_name__(key);
    return Ok(cmd.sadd(channel_name.as_ref(), value).await?);
  }
  async fn __srem__(
    &self,
    key: Arc<String>,
    value: Arc<dyn ToRedisArgs>,
  ) -> KVSResult<usize> {
    let mut cmd = self.__commands__();
    let channel_name = self.__channel_name__(key);
    return Ok(cmd.srem(channel_name.as_ref(), value).await?);
  }
  async fn __smembers__(
    &self,
    key: Arc<String>,
  ) -> KVSResult<Vec<Arc<dyn FromRedisValue>>> {
    let mut cmd = self.__commands__();
    let channel_name = self.__channel_name__(key);
    let values: Vec<Arc<dyn FromRedisValue>> = cmd
      .smembers::<_, Vec<Arc<dyn FromRedisValue>>>(channel_name.as_ref())
      .await?;
    return Ok(values);
  }
}
