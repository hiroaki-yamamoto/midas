use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, FromRedisValue};

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Get<T, V>: Base<T> + ChannelName
where
  T: Commands + Send,
  V: FromRedisValue,
{
  async fn get(&self, key: &str) -> KVSResult<V> {
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    let channel_name = self.channel_name(key);
    return Ok(cmd.get(channel_name).await?);
  }
}
