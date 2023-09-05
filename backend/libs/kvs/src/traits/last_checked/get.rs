use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::errors::KVSResult;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use super::base::Base;
use crate::traits::normal::Get as NormalGet;

#[async_trait]
pub trait Get<T, V>: Base<T> + NormalGet<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  async fn get(
    &self,
    key: impl AsRef<str> + Display + Send + Sync,
  ) -> KVSResult<V> {
    let value = NormalGet::get(self, &key).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(value);
  }
}
