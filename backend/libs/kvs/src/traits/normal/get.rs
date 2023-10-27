use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, FromRedisValue};

use ::errors::KVSResult;

use crate::traits::base::Get as Base;

#[async_trait]
pub trait Get<T, V>: Base<T, V>
where
  T: Commands + Send,
  V: FromRedisValue,
{
  async fn get(&self, key: Arc<String>) -> KVSResult<V> {
    return self.__get__(key).await;
  }
}
