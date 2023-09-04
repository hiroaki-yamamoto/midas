use ::std::fmt::Display;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Expiration<T, V>: Base<T, V> + ChannelName
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  async fn expire(
    &self,
    key: impl AsRef<str> + Display + Send,
    dur: Duration,
  ) -> KVSResult<bool> {
    let dur_mils = dur.as_millis() as usize;
    let commands = self.commands().lock().await;
    let channel_name = self.channel_name(key);
    if commands.pexpire::<_, u16>(channel_name, dur_mils)? == 1 {
      return Ok(true);
    } else {
      return Ok(false);
    };
  }
}
