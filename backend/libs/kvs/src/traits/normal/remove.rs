use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue};

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Remove<T>: Base<T> + ChannelName
where
  T: Commands + Send,
{
  async fn del<R>(&self, key: &str) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    let channel_name = self.channel_name(key);
    return Ok(cmd.del(channel_name)?);
  }
}
