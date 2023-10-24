use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Exist<T>: Base<T> + ChannelName
where
  T: Commands + Send,
{
  async fn __exists__(&self, key: &str) -> KVSResult<bool> {
    let channel_name = self.__channel_name__(key);
    let mut cmd = self.__commands__();
    // let mut cmd = cmd.lock().await;
    return Ok(cmd.exists(channel_name).await?);
  }
}
