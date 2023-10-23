use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use super::base::Base;
use crate::traits::base::Remove as BaseRemove;

#[async_trait]
pub trait Remove<T>: Base<T> + BaseRemove<T>
where
  T: Commands + Send,
{
  async fn del(&self, keys: &[Arc<str>]) -> KVSResult<usize> {
    let ret = BaseRemove::del(self, keys).await?;
    let _ = self.del_last_checked(keys).await?;
    return Ok(ret);
  }
}
