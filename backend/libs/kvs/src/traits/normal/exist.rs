use ::std::sync::Arc;

use ::async_trait::async_trait;

use ::errors::KVSResult;

use crate::traits::base::Exist as BaseExist;

#[async_trait]
pub trait Exist: BaseExist {
  async fn exists(&self, key: Arc<String>) -> KVSResult<bool> {
    return self.__exists__(key).await;
  }
}
