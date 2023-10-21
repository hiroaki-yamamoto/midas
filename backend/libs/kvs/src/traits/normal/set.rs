use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, FromRedisValue, ToRedisArgs};

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
    key: &str,
    value: V,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    return Self::set(&self, key, value, opt).await;
  }
}
