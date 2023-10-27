use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, SetOptions, ToRedisArgs};

use ::errors::KVSResult;

use super::{Base, ChannelName};
use crate::options::WriteOption;

#[async_trait]
pub trait Set: Base + ChannelName {
  type Value: ToRedisArgs + Send + Sync;

  async fn __set__(
    &self,
    key: Arc<String>,
    value: Self::Value,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<bool> {
    let channel_name = self.__channel_name__(key);
    let mut cmds = self.__commands__();
    // let mut cmds = cmds.lock().await;
    let result = if let Some(opt) = opt.into() {
      let opt: SetOptions = opt.into();
      cmds.set_options(channel_name.as_ref(), value, opt).await
    } else {
      cmds.set(channel_name.as_ref(), value).await
    };
    return Ok(result?);
  }
}
