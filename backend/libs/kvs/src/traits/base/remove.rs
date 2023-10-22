use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, FromRedisValue};

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Remove<T>: Base<T> + ChannelName
where
  T: Commands + Send,
{
  async fn del<R>(&self, keys: &[Arc<str>]) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let mut cmd = self.commands();
    // let mut cmd = cmd.lock().await;
    let channel_names: Vec<String> =
      keys.into_iter().map(|key| self.channel_name(key)).collect();
    return Ok(cmd.del(channel_names).await?);
  }
}
