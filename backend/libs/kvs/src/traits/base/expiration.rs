use ::std::time::Duration;

use ::async_trait::async_trait;
use ::redis::AsyncCommands as Commands;

use ::errors::KVSResult;

use super::{Base, ChannelName};

#[async_trait]
pub trait Expiration<T>: Base<T> + ChannelName
where
  T: Commands + Send,
{
  async fn expire(&self, key: &str, dur: Duration) -> KVSResult<bool> {
    let dur_mils = dur.as_millis() as usize;
    let mut cmd = self.commands();
    // let mut cmd = cmd.lock().await;
    let channel_name = self.channel_name(key);
    if cmd.pexpire::<_, u16>(channel_name, dur_mils).await? == 1 {
      return Ok(true);
    } else {
      return Ok(false);
    };
  }
}
