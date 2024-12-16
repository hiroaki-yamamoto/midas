use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::symbol::OptExecution;

#[async_trait]
pub trait Incr: OptExecution {
  async fn incr(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
    delta: i64,
    opt: Option<WriteOption>,
  ) -> KVSResult<()> {
    let channel_name = self.channel_name(exchange.clone(), symbol.clone());
    let mut cmds = self.__commands__();
    cmds.incr(channel_name.as_ref(), delta).await?;
    self.execute_opt(exchange, symbol, opt).await?;
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
