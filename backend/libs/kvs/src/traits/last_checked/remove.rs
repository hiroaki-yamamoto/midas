use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue};

use ::errors::KVSResult;

use super::base::Base;
use crate::traits::normal::Remove as NormalRemove;

#[async_trait]
pub trait Remove<T>: Base<T> + NormalRemove<T>
where
  T: Commands + Send,
{
  async fn del<R>(&self, keys: &[Arc<str>]) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let ret = NormalRemove::del(self, keys).await?;
    let _ = self.del_last_checked(keys).await?;
    return Ok(ret);
  }
}
