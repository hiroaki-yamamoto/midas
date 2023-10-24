use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use crate::traits::base::Remove as Base;

#[async_trait]
pub trait Remove<T>: Base<T>
where
  T: Commands + Send,
{
  async fn del(&self, keys: &[Arc<str>]) -> KVSResult<usize> {
    return self.__del__(keys).await;
  }
}
