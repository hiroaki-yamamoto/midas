use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::errors::KVSResult;

use super::base::Base;
use crate::traits::base::Get as BaseGet;

#[async_trait]
pub trait Get: Base + BaseGet {
  async fn get(&self, key: Arc<String>) -> KVSResult<Self::Value> {
    let value = self.__get__(key.clone()).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(value);
  }
}
