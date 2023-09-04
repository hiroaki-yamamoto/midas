use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::normal::Set as NormalSet;

use super::base::Base;

#[async_trait]
pub trait Set<T, V>: Base<T> + NormalSet<T, V>
where
  T: Commands + Send,
  V: ToRedisArgs + Send,
{
  async fn set<R>(
    &self,
    key: impl AsRef<str> + Display + Send,
    value: V,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let ret = NormalSet::set(self, key, value, opt.clone()).await?;
    self.flag_last_checked(key, opt.into()).await?;
    return Ok(ret);
  }
}
