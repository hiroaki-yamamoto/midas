use ::async_trait::async_trait;
use ::futures::future::TryFutureExt;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use crate::options::{WriteOption, WriteOptionTrait};
use crate::traits::base::Base;

use super::channel_name::ChannelName;

#[async_trait]
pub trait Incr<T>: ChannelName + Base<T>
where
  T: Commands + Send,
{
  async fn incr(
    &self,
    exchange: &str,
    symbol: &str,
    delta: i64,
    opt: Option<WriteOption>,
  ) -> KVSResult<()> {
    let channel_name = self.channel_name(exchange, symbol);
    let cmds = self.commands();
    let mut cmds = cmds.lock().await;
    return Ok(
      cmds
        .incr(&channel_name, delta)
        .and_then(|_: ()| async {
          return opt.execute(self.commands(), &channel_name).await;
        })
        .await?,
    );
  }

  async fn reset(&self, exchange: &str, symbol: &str) -> KVSResult<()> {
    let channel_name = self.channel_name(exchange, symbol);
    return Ok(self.commands().lock().await.set(channel_name, 0).await?);
  }
}
