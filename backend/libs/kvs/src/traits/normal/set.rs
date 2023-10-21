use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::base::Set as Base;

#[async_trait]
pub trait Set<T, V>: Base<T, V>
where
  T: Commands + Send,
  for<'a> V: ToRedisArgs + Send + 'a,
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
