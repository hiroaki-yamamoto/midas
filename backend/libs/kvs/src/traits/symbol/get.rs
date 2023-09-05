use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::errors::KVSResult;
use ::redis::{Commands, FromRedisValue};

use super::channel_name::ChannelName;
use crate::traits::normal::Base;

#[async_trait]
pub trait Get<T, V>: Base<T> + ChannelName
where
  T: Commands + Send,
  V: FromRedisValue,
{
  async fn get(
    &self,
    exchange: impl AsRef<str> + Display + Send,
    symbol: impl AsRef<str> + Display + Send,
  ) -> KVSResult<V> {
    let channel_name = self.channel_name(exchange, symbol);
    return Ok(self.commands().lock().await.get(channel_name)?);
  }
}
