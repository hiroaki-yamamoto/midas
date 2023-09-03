use ::std::fmt::Display;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, SetOptions, ToRedisArgs};

use ::errors::KVSResult;

use super::NormalStoreBase;
use crate::options::WriteOption;

#[async_trait]
pub trait Set<T, V>: NormalStoreBase<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
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
    let mut cmds = self.commands().lock().await;
    let result = if let Some(opt) = opt.into() {
      let opt: SetOptions = opt.into();
      cmds.set_options(&channel_name, value, opt)
    } else {
      cmds.set(&channel_name, value)
    };
    return Ok(result?);
  }
}
