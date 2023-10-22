use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Exist<T>: Base<T> + ChannelName
where
  T: Commands + Send,
{
  async fn exists(&self, key: &str) -> KVSResult<bool> {
    let channel_name = self.channel_name(key);
    let mut cmd = self.commands();
    // let mut cmd = cmd.lock().await;
    return Ok(cmd.exists(channel_name).await?);
  }
}
