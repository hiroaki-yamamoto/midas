use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use crate::traits::base::Exist as BaseExist;

#[async_trait]
pub trait Exist<T>: BaseExist<T>
where
  T: Commands + Send,
{
  async fn exists(&self, key: Arc<String>) -> KVSResult<bool> {
    return self.__exists__(key).await;
  }
}
