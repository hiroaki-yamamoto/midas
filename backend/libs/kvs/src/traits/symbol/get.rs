use ::async_trait::async_trait;
use ::errors::KVSResult;
use ::redis::{AsyncCommands as Commands, FromRedisValue};

use super::channel_name::ChannelName;
use crate::traits::base::Base;

#[async_trait]
pub trait Get<T, V>: Base<T> + ChannelName
where
  T: Commands + Send + Sync,
  V: FromRedisValue,
{
  async fn get(&self, exchange: &str, symbol: &str) -> KVSResult<V> {
    let channel_name = self.channel_name(exchange, symbol);
    return Ok(self.commands().get(channel_name).await?);
  }
}
