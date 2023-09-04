use ::std::fmt::Display;
use ::std::num::NonZeroUsize;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use super::base::Base;
use crate::options::WriteOption;
use crate::traits::normal::ListOp as NormalListOp;

#[async_trait]
pub trait ListOp<T, V>: Base<T, V> + NormalListOp<T, V> + Send
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
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

  async fn lpop<R>(
    &self,
    key: impl AsRef<str> + Display + Send + Sync,
    count: Option<NonZeroUsize>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let ret = NormalListOp::lpop(self, &key, count).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }
}
