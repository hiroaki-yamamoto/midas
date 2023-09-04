use ::std::fmt::Display;
use ::std::time::SystemTime;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Local, LocalResult, TimeZone};
use ::errors::{KVSError, KVSResult};
use ::redis::{Commands, FromRedisValue, SetOptions, ToRedisArgs};

use crate::options::WriteOption;
use crate::traits::normal::{Base as NormalBase, ChannelName};

#[async_trait]
pub trait Base<T, V>: NormalBase<T, V> + ChannelName
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  fn get_timestamp_channel(&self, key: impl AsRef<str> + Display) -> String {
    return format!("last_check_timestamp:{}", self.channel_name(key));
  }

  async fn get_last_checked(
    &self,
    key: impl AsRef<str> + Display + Send,
  ) -> KVSResult<SystemTime> {
    let key = self.get_timestamp_channel(key);
    let cmd = self.commands().lock().await;
    let timestamp: i64 = cmd.get(key)?;
    let datetime: DateTime<Local> = match Local.timestamp_opt(timestamp, 0) {
      LocalResult::Single(dt) => dt,
      LocalResult::None => {
        return Err(KVSError::TimestampError(timestamp));
      }
      LocalResult::Ambiguous(_, _) => {
        return Err(KVSError::TimestampError(timestamp));
      }
    };
    return Ok(datetime.into());
  }

  async fn flag_last_checked<R>(
    &self,
    key: impl AsRef<str> + Display + Send,
    opt: Option<WriteOption>,
  ) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let key = self.get_timestamp_channel(key);
    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_secs();
    let opt: Option<SetOptions> = opt.map(|opt| opt.into());
    let mut cmd = self.commands().lock().await;
    return Ok(match opt {
      Some(opt) => cmd.set_options(key, now, opt)?,
      None => cmd.set(key, now)?,
    });
  }

  async fn del_last_checked<R>(
    &self,
    key: impl AsRef<str> + Display + Send,
  ) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let key = self.get_timestamp_channel(key);
    let mut cmd = self.commands().lock().await;
    return Ok(cmd.del(key)?);
  }
}
