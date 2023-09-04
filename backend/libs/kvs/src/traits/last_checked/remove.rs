use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use super::base::Base;
use crate::traits::normal::Remove as NormalRemove;

#[async_trait]
pub trait Remove<T, V>: Base<T, V> + NormalRemove<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  async fn del<R>(
    &self,
    key: &(impl AsRef<str> + Display + Send + Sync),
  ) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let ret = NormalRemove::del(self, key).await?;
    let _ = self.del_last_checked(key).await?;
    return Ok(ret);
  }
}
