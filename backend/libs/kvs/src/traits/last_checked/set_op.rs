use ::async_trait::async_trait;
use ::std::sync::Arc;

use super::Base;
use crate::redis::{FromRedisValue, ToRedisArgs};
use crate::traits::base::SetOp as BaseSetOp;
use crate::KVSResult;

#[async_trait]
pub trait SetOp: BaseSetOp + Base {
  async fn sadd(
    &self,
    key: Arc<String>,
    value: Arc<dyn ToRedisArgs>,
  ) -> KVSResult<usize> {
    let val = self.__sadd__(key.clone(), value).await;
    Self::flag_last_checked(self, key, None).await?;
    return val;
  }
  async fn srem(
    &self,
    key: Arc<String>,
    value: Arc<dyn ToRedisArgs>,
  ) -> KVSResult<usize> {
    let val = self.__srem__(key.clone(), value).await;
    Self::flag_last_checked(self, key, None).await?;
    return val;
  }
  async fn smembers(
    &self,
    key: Arc<String>,
  ) -> KVSResult<Vec<Arc<dyn FromRedisValue>>> {
    let val = self.__smembers__(key.clone()).await;
    Self::flag_last_checked(self, key, None).await?;
    return val;
  }
}
