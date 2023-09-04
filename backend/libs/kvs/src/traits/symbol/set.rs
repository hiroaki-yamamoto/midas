use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::errors::KVSResult;
use ::redis::{Commands, FromRedisValue, SetOptions, ToRedisArgs};

use super::channel_name::ChannelName;
use crate::options::WriteOption;
use crate::traits::normal::Base;

#[async_trait]
pub trait Set<T, V>: Base<T> + ChannelName
where
  T: Commands + Send,
  V: ToRedisArgs + Send,
{
  async fn set<R>(
    &self,
    exchange: impl AsRef<str> + Display + Send,
    symbol: impl AsRef<str> + Display + Send,
    value: V,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(exchange, symbol);
    let mut cmds = self.commands().lock().await;
    let result = if let Some(opt) = opt {
      let opt: SetOptions = opt.into();
      cmds.set_options(&channel_name, value, opt)
    } else {
      cmds.set(&channel_name, value)
    };
    return Ok(result?);
  }
}
