use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use super::channel_name::ChannelName;
use crate::traits::base::Base;

#[async_trait]
pub trait Remove: Base + ChannelName {
  async fn del(
    &self,
    exchange: Arc<String>,
    symbols: Arc<[Arc<String>]>,
  ) -> KVSResult<()> {
    let channel_names: Vec<String> = symbols
      .into_iter()
      .map(|symbol| {
        (*self.channel_name(exchange.clone(), symbol.clone())).clone()
      })
      .collect();
    let mut cmd = self.__commands__();
    // let mut cmd = cmd.lock().await;
    return Ok(cmd.del(channel_names).await?);
  }
}
