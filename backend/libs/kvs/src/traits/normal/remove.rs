use ::std::sync::Arc;

use ::async_trait::async_trait;

use ::errors::KVSResult;

use crate::traits::base::Remove as Base;

#[async_trait]
pub trait Remove: Base {
  async fn del(&self, keys: Arc<[Arc<String>]>) -> KVSResult<usize> {
    return self.__del__(keys).await;
  }
}
