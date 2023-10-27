use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::errors::KVSResult;
use ::redis::{AsyncCommands as Commands, SetOptions, ToRedisArgs};

use super::channel_name::ChannelName;
use crate::options::WriteOption;
use crate::traits::base::Base;

#[async_trait]
pub trait Set: Base + ChannelName {
  async fn set(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
    value: Arc<dyn ToRedisArgs>,
    opt: Option<WriteOption>,
  ) -> KVSResult<bool> {
    let channel_name = self.channel_name(exchange, symbol);
    let mut cmds = self.__commands__();
    // let mut cmds = cmds.lock().await;
    let result = if let Some(opt) = opt {
      let opt: SetOptions = opt.into();
      cmds.set_options(channel_name.as_ref(), value, opt)
    } else {
      cmds.set(channel_name.as_ref(), value)
    }
    .await;
    return Ok(result?);
  }
}
