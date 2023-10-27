use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::ToRedisArgs;

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::base::Set as Base;

#[async_trait]
pub trait Set: Base {
  async fn set(
    &self,
    key: Arc<String>,
    value: Arc<dyn ToRedisArgs>,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<bool> {
    return self.__set__(key, value, opt).await;
  }
}
