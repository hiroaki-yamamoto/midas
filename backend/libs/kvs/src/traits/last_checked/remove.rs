use ::std::sync::Arc;

use ::async_trait::async_trait;

use ::errors::KVSResult;

use super::base::Base;
use crate::traits::base::Remove as BaseRemove;

#[async_trait]
pub trait Remove: Base + BaseRemove {
  async fn del(&self, keys: Arc<[Arc<String>]>) -> KVSResult<usize> {
    let ret = self.__del__(keys.clone()).await?;
    let _ = self.del_last_checked(keys).await?;
    return Ok(ret);
  }
}
