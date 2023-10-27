use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, ToRedisArgs};

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::base::Set as Base;

#[async_trait]
pub trait Set<T, V>: Base<T, V>
where
  T: Commands + Send,
  for<'async_trait> V: ToRedisArgs + Send + Sync + 'async_trait,
{
  async fn set<R>(
    &self,
    key: Arc<String>,
    value: V,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<bool> {
    return self.__set__(key, value, opt).await;
  }
}
