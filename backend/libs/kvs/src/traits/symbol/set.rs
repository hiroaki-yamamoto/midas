use ::async_trait::async_trait;
use ::errors::KVSResult;
use ::redis::{Commands, FromRedisValue, SetOptions, ToRedisArgs};

use super::channel_name::ChannelName;
use crate::options::WriteOption;
use crate::traits::base::Base;

#[async_trait]
pub trait Set<T, V>: Base<T> + ChannelName
where
  T: Commands + Send,
  for<'a> V: ToRedisArgs + Send + 'a,
{
  async fn set<R>(
    &self,
    exchange: &str,
    symbol: &str,
    value: V,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(exchange, symbol);
    let cmds = self.commands();
    let mut cmds = cmds.lock().await;
    let result = if let Some(opt) = opt {
      let opt: SetOptions = opt.into();
      cmds.set_options(&channel_name, value, opt)
    } else {
      cmds.set(&channel_name, value)
    };
    return Ok(result?);
  }
}
