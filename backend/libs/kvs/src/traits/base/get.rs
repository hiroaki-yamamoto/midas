use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, FromRedisValue};

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Get: Base + ChannelName {
  type Value: FromRedisValue + Send + Sync;
  async fn __get__(&self, key: Arc<String>) -> KVSResult<Self::Value> {
    let mut cmd = self.__commands__();
    let channel_name = self.__channel_name__(key);
    return Ok(cmd.get(channel_name.as_ref()).await?);
  }
}
