use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::redis::Commands;

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Exist<T>: Base<T> + ChannelName
where
  T: Commands + Send,
{
  async fn exists(
    &self,
    key: impl AsRef<str> + Display + Send,
  ) -> KVSResult<bool> {
    let channel_name = self.channel_name(key);
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.exists(channel_name)?);
  }
}
