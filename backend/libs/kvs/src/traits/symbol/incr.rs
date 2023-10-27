use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::futures::future::TryFutureExt;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use crate::options::{WriteOption, WriteOptionTrait};
use crate::traits::base::Base;

use super::channel_name::ChannelName;

#[async_trait]
pub trait Incr: ChannelName + Base {
  async fn incr(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
    delta: i64,
    opt: Option<WriteOption<Self::Commands>>,
  ) -> KVSResult<()> {
    let channel_name = self.channel_name(exchange, symbol);
    let mut cmds = self.__commands__();
    // let mut cmds = cmds.lock().await;
    return Ok(
      cmds
        .incr(channel_name.as_ref(), delta)
        .and_then(|_: ()| async {
          return opt.execute(self.__commands__(), channel_name.clone()).await;
        })
        .await?,
    );
  }

  async fn reset(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
  ) -> KVSResult<()> {
    let channel_name = self.channel_name(exchange, symbol);
    return Ok(self.__commands__().set(channel_name.as_ref(), 0).await?);
  }
}
