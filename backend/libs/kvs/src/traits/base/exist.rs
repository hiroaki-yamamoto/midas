use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;
use ::std::sync::Arc;

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Exist<T>: Base<T> + ChannelName
where
  T: Commands + Send,
{
  async fn __exists__(&self, key: Arc<String>) -> KVSResult<bool> {
    let channel_name = self.__channel_name__(key.into());
    let mut cmd = self.__commands__();
    // let mut cmd = cmd.lock().await;
    return Ok(cmd.exists(channel_name.as_ref()).await?);
  }
}
