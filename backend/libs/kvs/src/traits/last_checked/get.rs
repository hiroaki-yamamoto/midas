use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::errors::KVSResult;
use ::redis::{AsyncCommands as Commands, FromRedisValue, ToRedisArgs};

use super::base::Base;
use crate::traits::base::Get as BaseGet;

#[async_trait]
pub trait Get<T, V>: Base<T> + BaseGet<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  async fn get(&self, key: Arc<String>) -> KVSResult<V> {
    let value = self.__get__(key.clone()).await?;
    self.flag_last_checked(key, None).await?;
    return Ok(value);
  }
}
