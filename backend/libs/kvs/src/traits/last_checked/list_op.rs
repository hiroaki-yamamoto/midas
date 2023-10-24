use ::std::num::NonZeroUsize;

use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use super::base::Base;
use crate::options::WriteOption;
use crate::traits::base::ListOp as BaseListOp;

#[async_trait]
pub trait ListOp<T, V>: Base<T> + BaseListOp<T, V>
where
  T: Commands + Send,
  for<'async_trait> V:
    FromRedisValue + ToRedisArgs + Send + Sync + 'async_trait,
{
  async fn lpush(
    &self,
    key: &str,
    value: Vec<V>,
    opt: Option<WriteOption>,
  ) -> KVSResult<usize> {
    let ret = self.__lpush__(key, value, opt.clone()).await?;
    self.flag_last_checked(key, opt.into()).await?;
    return Ok(ret);
  }

  async fn lpop(&self, key: &str, count: Option<NonZeroUsize>) -> KVSResult<V> {
    let ret = self.__lpop__(key, count).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }

  async fn lrem(&self, key: &str, count: isize, elem: V) -> KVSResult<usize> {
    let ret = self.__lrem__(key, count, elem).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }

  async fn lrange(
    &self,
    key: &str,
    start: isize,
    stop: isize,
  ) -> KVSResult<Vec<V>> {
    let ret = self.__lrange__(key, start, stop).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }

  async fn llen(&self, key: &str) -> KVSResult<usize> {
    let ret = self.__llen__(key).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }
}
