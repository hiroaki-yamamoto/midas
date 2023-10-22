use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, FromRedisValue, ToRedisArgs};
use ::std::num::NonZeroUsize;

use ::errors::KVSResult;

use crate::traits::base::{ListOp as Base, Lock};

use crate::WriteOption;

#[async_trait]
pub trait ListOp<T, V>: Base<T, V> + Lock<T>
where
  T: Commands + Send,
  for<'async_trait> V:
    FromRedisValue + ToRedisArgs + Send + Sync + 'async_trait,
{
  async fn lpush(
    &self,
    key: &str,
    value: Vec<V>,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<usize> {
    return Base::lpush(self, key, value, opt).await;
  }

  async fn lpop(&self, key: &str, count: Option<NonZeroUsize>) -> KVSResult<V> {
    return Base::lpop(self, key, count).await;
  }

  async fn lrem(&self, key: &str, count: isize, elem: V) -> KVSResult<usize> {
    return Base::lrem(self, key, count, elem).await;
  }

  async fn lrange(
    &self,
    key: &str,
    start: isize,
    stop: isize,
  ) -> KVSResult<Vec<V>> {
    return Base::lrange(self, key, start, stop).await;
  }

  async fn llen(&self, key: &str) -> KVSResult<usize> {
    return Base::llen(self, key).await;
  }
}
