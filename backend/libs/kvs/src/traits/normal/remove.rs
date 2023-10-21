use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue};

use ::errors::KVSResult;

use crate::traits::base::Remove as Base;

#[async_trait]
pub trait Remove<T>: Base<T>
where
  T: Commands + Send,
{
  async fn del<R>(&self, keys: &[Arc<str>]) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    return Base::del(&self, keys).await;
  }
}
