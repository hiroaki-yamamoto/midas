use ::std::sync::Arc;

use ::async_trait::async_trait;

use ::errors::KVSResult;

use crate::traits::base::Get as Base;

#[async_trait]
pub trait Get: Base {
  async fn get(&self, key: Arc<String>) -> KVSResult<Option<Self::Value>> {
    return self.__get__(key).await;
  }
}
