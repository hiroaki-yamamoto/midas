use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use super::ChannelName;
use crate::traits::base::Base;

#[async_trait]
pub trait Expiration: Base + ChannelName {
  async fn expire(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
    dur: Duration,
  ) -> KVSResult<bool> {
    let dur_mils = dur.as_millis() as i64;
    let mut cmd = self.__commands__();
    let channel_name = self.channel_name(exchange, symbol);
    if cmd
      .pexpire::<_, u16>(channel_name.as_ref(), dur_mils)
      .await?
      == 1
    {
      return Ok(true);
    } else {
      return Ok(false);
    };
  }
}
