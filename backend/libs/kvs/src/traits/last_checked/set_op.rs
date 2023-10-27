use ::async_trait::async_trait;
use ::std::sync::Arc;

use super::Base;
use crate::traits::base::SetOp as BaseSetOp;
use crate::KVSResult;

#[async_trait]
pub trait SetOp: BaseSetOp + Base {
  async fn sadd(
    &self,
    key: Arc<String>,
    value: Self::Value,
  ) -> KVSResult<usize> {
    let val = self.__sadd__(key.clone(), value).await;
    Self::flag_last_checked(self, key, Arc::new(None)).await?;
    return val;
  }
  async fn srem(
    &self,
    key: Arc<String>,
    value: Self::Value,
  ) -> KVSResult<usize> {
    let val = self.__srem__(key.clone(), value).await;
    Self::flag_last_checked(self, key, Arc::new(None)).await?;
    return val;
  }
  async fn smembers(&self, key: Arc<String>) -> KVSResult<Vec<Self::Value>> {
    let val = self.__smembers__(key.clone()).await;
    Self::flag_last_checked(self, key, Arc::new(None)).await?;
    return val;
  }
}
