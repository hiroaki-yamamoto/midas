use ::async_trait::async_trait;
use ::redis::Commands;

use ::errors::KVSResult;

use super::channel_name::ChannelName;
use crate::traits::normal::Base;

#[async_trait]
pub trait Remove<T>: Base<T> + ChannelName
where
  T: Commands + Send,
{
  async fn del(&self, exchange: &str, symbol: &str) -> KVSResult<()> {
    let channel_name = self.channel_name(exchange, symbol);
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.del(channel_name)?);
  }
}
