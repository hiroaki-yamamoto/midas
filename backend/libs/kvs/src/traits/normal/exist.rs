use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use super::NormalStoreBase;

#[async_trait]
pub trait Exist<T, V>: NormalStoreBase<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  async fn exists(
    &self,
    key: impl AsRef<str> + Display + Send,
  ) -> KVSResult<bool> {
    let channel_name = self.channel_name(key);
    let cmd = self.commands().lock().await;
    return Ok(cmd.exists(channel_name)?);
  }
}
