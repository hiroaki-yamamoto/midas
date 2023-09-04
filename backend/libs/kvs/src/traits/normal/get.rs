use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};
use ::std::fmt::Display;

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Get<T, V>: Base<T, V> + ChannelName
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  async fn get(&self, key: impl AsRef<str> + Display + Send) -> KVSResult<V> {
    let mut cmd = self.commands().lock().await;
    let channel_name = self.channel_name(key);
    return Ok(cmd.get(channel_name)?);
  }
}
