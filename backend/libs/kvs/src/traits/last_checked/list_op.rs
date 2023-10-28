use ::std::num::NonZeroUsize;
use ::std::sync::Arc;

use ::async_trait::async_trait;

use ::errors::KVSResult;

use super::base::Base;
use crate::options::WriteOption;
use crate::traits::base::ListOp as BaseListOp;

#[async_trait]
pub trait ListOp: Base + BaseListOp {
  async fn lpush(
    &self,
    key: Arc<String>,
    value: Vec<Self::Value>,
    opt: Option<WriteOption>,
  ) -> KVSResult<usize> {
    let ret = self
      .__lpush__(key.clone(), value, opt.clone().into())
      .await?;
    self.flag_last_checked(key, opt.clone()).await?;
    return Ok(ret);
  }

  async fn lpop(
    &self,
    key: Arc<String>,
    count: Option<NonZeroUsize>,
  ) -> KVSResult<Self::Value> {
    let ret = self.__lpop__(key.clone(), count).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }

  async fn lrem(
    &self,
    key: Arc<String>,
    count: isize,
    elem: Self::Value,
  ) -> KVSResult<usize> {
    let ret = self.__lrem__(key.clone(), count, elem).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }

  async fn lrange(
    &self,
    key: Arc<String>,
    start: isize,
    stop: isize,
  ) -> KVSResult<Vec<Self::Value>> {
    let ret = self.__lrange__(key.clone(), start, stop).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }

  async fn llen(&self, key: Arc<String>) -> KVSResult<usize> {
    let ret = self.__llen__(key.clone()).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(ret);
  }
}
