use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Remove<T, V>: Base<T, V> + ChannelName
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  async fn del<R>(&self, key: impl AsRef<str> + Send + Display) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let mut cmd = self.commands().lock().await;
    let channel_name = self.channel_name(key);
    return Ok(cmd.del(channel_name)?);
  }
}
