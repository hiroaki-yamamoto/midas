use ::async_trait::async_trait;
use ::redis::Commands;

use ::errors::KVSResult;

use crate::traits::base::Exist as BaseExist;

#[async_trait]
pub trait Exist<T>: BaseExist<T>
where
  T: Commands + Send,
{
  async fn exists(&self, key: &str) -> KVSResult<bool> {
    return BaseExist::exists(self, key).await;
  }
}
