use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::base::OptExecution;

use super::channel_name::ChannelName;

#[async_trait]
pub trait Incr: ChannelName + OptExecution {
  async fn incr(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
    delta: i64,
    opt: Option<WriteOption>,
  ) -> KVSResult<()> {
    let channel_name = self.channel_name(exchange, symbol);
    let mut cmds = self.__commands__();
    // let mut cmds = cmds.lock().await;
    cmds.incr(channel_name.as_ref(), delta).await?;
    self.__execute_opt__(channel_name, opt).await?;
    return Ok(());
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
