use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use super::NormalStoreBase;

#[async_trait]
pub trait Remove<T, V>: NormalStoreBase<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  async fn del(&self, key: impl AsRef<str> + Send + Display) -> KVSResult<()> {
    let mut cmd = self.commands().lock().await;
    let channel_name = self.channel_name(key);
    return Ok(cmd.del(channel_name)?);
  }
}
