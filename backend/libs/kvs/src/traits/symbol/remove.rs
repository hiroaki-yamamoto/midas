use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use super::channel_name::ChannelName;
use crate::traits::base::Base;

#[async_trait]
pub trait Remove<T>: Base<T> + ChannelName
where
  T: Commands + Send + Sync,
{
  async fn del(&self, exchange: &str, symbols: &[Arc<str>]) -> KVSResult<()> {
    let channel_names: Vec<String> = symbols
      .into_iter()
      .map(|symbol| self.channel_name(exchange, symbol))
      .collect();
    let mut cmd = self.commands();
    // let mut cmd = cmd.lock().await;
    return Ok(cmd.del(channel_names).await?);
  }
}
