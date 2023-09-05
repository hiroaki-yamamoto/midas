use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, SetOptions, ToRedisArgs};

use ::errors::KVSResult;

use super::{Base, ChannelName};
use crate::options::WriteOption;

#[async_trait]
pub trait Set<T, V>: Base<T> + ChannelName
where
  T: Commands + Send,
  for<'a> V: ToRedisArgs + Send + 'a,
{
  async fn set<R>(
    &self,
    key: impl AsRef<str> + Send + Display,
    value: V,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    let cmds = self.commands();
    let mut cmds = cmds.lock().await;
    let result = if let Some(opt) = opt.into() {
      let opt: SetOptions = opt.into();
      cmds.set_options(&channel_name, value, opt)
    } else {
      cmds.set(&channel_name, value)
    };
    return Ok(result?);
  }
}
