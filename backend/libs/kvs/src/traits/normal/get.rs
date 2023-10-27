use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::FromRedisValue;

use ::errors::KVSResult;

use crate::traits::base::Get as Base;

#[async_trait]
pub trait Get: Base {
  async fn get(&self, key: Arc<String>) -> KVSResult<Arc<dyn FromRedisValue>> {
    return self.__get__(key).await;
  }
}
