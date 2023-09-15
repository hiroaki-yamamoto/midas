use ::async_trait::async_trait;
use ::redis::Commands;

use ::errors::KVSResult;

use crate::options::{WriteOption, WriteOptionTrait};
use crate::traits::normal::Base;

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
    return Ok(cmds.incr(&channel_name, delta).and_then(|_: ()| {
      return opt.execute(&mut cmds, &channel_name);
    })?);
  }

  async fn reset(&self, exchange: &str, symbol: &str) -> KVSResult<()> {
    let channel_name = self.channel_name(exchange, symbol);
    return Ok(self.commands().lock().await.set(channel_name, 0)?);
  }
}
