use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::std::num::NonZeroUsize;

use ::errors::KVSResult;

use crate::traits::base::ListOp as Base;

use crate::WriteOption;

#[async_trait]
pub trait ListOp: Base {
  async fn lpush(
    &self,
    key: Arc<String>,
    value: Vec<Self::Value>,
    opt: Option<WriteOption<Self::Commands>>,
  ) -> KVSResult<usize> {
    return self.__lpush__(key, value, Arc::new(opt)).await;
  }

  async fn lpop(
    &self,
    key: Arc<String>,
    count: Option<NonZeroUsize>,
  ) -> KVSResult<Self::Value> {
    return self.__lpop__(key, count).await;
  }

  async fn lrem(
    &self,
    key: Arc<String>,
    count: isize,
    elem: Self::Value,
  ) -> KVSResult<usize> {
    return self.__lrem__(key, count, elem).await;
  }

  async fn lrange(
    &self,
    key: Arc<String>,
    start: isize,
    stop: isize,
  ) -> KVSResult<Vec<Self::Value>> {
    return self.__lrange__(key, start, stop).await;
  }

  async fn llen(&self, key: Arc<String>) -> KVSResult<usize> {
    return self.__llen__(key).await;
  }
}
