use ::async_trait::async_trait;

use crate::redis::{AsyncCommands, FromRedisValue, RedisResult, ToRedisArgs};

use super::{Base, ChannelName};

#[async_trait]
pub trait SetOp<T, V>: Base<T> + ChannelName
where
  T: AsyncCommands,
  for<'a> V: ToRedisArgs + FromRedisValue + Send + Sync + 'a,
{
  async fn sadd(&self, key: &str, value: V) -> RedisResult<usize> {
    let mut cmd = self.commands();
    let channel_name = self.channel_name(key);
    return cmd.sadd(channel_name, value).await;
  }
  async fn srem(&self, key: &str, value: V) -> RedisResult<usize> {
    let mut cmd = self.commands();
    let channel_name = self.channel_name(key);
    return cmd.srem(channel_name, value).await;
  }
  async fn smembers(&self, key: &str) -> RedisResult<Vec<V>> {
    let mut cmd = self.commands();
    let channel_name = self.channel_name(key);
    let values: Vec<V> = cmd.smembers::<_, Vec<V>>(channel_name).await?;
    return Ok(values);
  }
}
