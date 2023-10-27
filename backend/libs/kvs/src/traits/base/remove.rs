use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Remove<T>: Base<T> + ChannelName
where
  T: Commands + Send,
{
  async fn __del__(&self, keys: Arc<[Arc<String>]>) -> KVSResult<usize> {
    let mut cmd = self.__commands__();
    // let mut cmd = cmd.lock().await;
    let channel_names: Vec<String> = keys
      .into_iter()
      .map(|key| (*self.__channel_name__(key.clone())).clone())
      .collect();
    return Ok(cmd.del(channel_names).await?);
  }
}
