use ::async_trait::async_trait;

use super::Base;
use crate::redis::{AsyncCommands as Commands, FromRedisValue, ToRedisArgs};
use crate::traits::base::SetOp as BaseSetOp;
use crate::KVSResult;

#[async_trait]
pub trait SetOp<T, V>: BaseSetOp<T, V> + Base<T>
where
  T: Commands,
  for<'a> V: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
{
  async fn sadd(&self, key: &str, value: V) -> KVSResult<usize> {
    let val = BaseSetOp::sadd(self, key, value).await;
    Self::flag_last_checked(self, key, None).await?;
    return val;
  }
  async fn srem(&self, key: &str, value: V) -> KVSResult<usize> {
    let val = BaseSetOp::srem(self, key, value).await;
    Self::flag_last_checked(self, key, None).await?;
    return val;
  }
  async fn smembers(&self, key: &str) -> KVSResult<Vec<V>> {
    let val = BaseSetOp::smembers(self, key).await;
    Self::flag_last_checked(self, key, None).await?;
    return val;
  }
}
