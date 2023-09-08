use ::std::fmt::Display;
use ::std::num::NonZeroUsize;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use super::base::Base;
use crate::options::WriteOption;
use crate::traits::normal::ListOp as NormalListOp;

#[async_trait]
pub trait ListOp<T, V>: Base<T> + NormalListOp<T, V>
where
  T: Commands + Send,
  for<'async_trait> V: FromRedisValue + ToRedisArgs + Send + 'async_trait,
{
  async fn lpush<R>(
    &self,
    key: impl AsRef<str> + Display + Send + Sync,
    value: V,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let ret = NormalListOp::lpush(self, &key, value, opt.clone()).await?;
    self.flag_last_checked(key, opt.into()).await?;
    return Ok(ret);
  }

  async fn lpop(
    &self,
    key: impl AsRef<str> + Display + Send + Sync,
    count: Option<NonZeroUsize>,
  ) -> KVSResult<V> {
    let ret = NormalListOp::lpop(self, &key, count).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }

  async fn lrange<R>(
    &self,
    key: impl AsRef<str> + Display + Send + Sync,
    start: isize,
    stop: isize,
  ) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let ret = NormalListOp::lrange(self, &key, start, stop).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }

  async fn llen<R>(
    &self,
    key: impl AsRef<str> + Display + Send + Sync,
  ) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let ret = NormalListOp::llen(self, &key).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }
}