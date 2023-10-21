use ::std::sync::Arc;
use ::std::time::SystemTime;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Local, LocalResult, TimeZone};
use ::errors::{KVSError, KVSResult};
use ::redis::{Commands, FromRedisValue, SetOptions};

use crate::options::WriteOption;
use crate::traits::base::{Base as BaseBase, ChannelName};

#[async_trait]
pub trait Base<T>: BaseBase<T> + ChannelName
where
  T: Commands + Send,
{
  fn get_timestamp_channel(&self, key: &str) -> String {
    return format!("last_check_timestamp:{}", self.channel_name(key));
  }

  fn convert_timestamp(timestamp: i64) -> KVSResult<SystemTime> {
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

  async fn get_last_checked(&self, key: &str) -> KVSResult<SystemTime> {
    let key = self.get_timestamp_channel(key);
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    let timestamp: i64 = cmd.get(key)?;
    return Ok(Self::convert_timestamp(timestamp)?);
  }

  async fn flag_last_checked<R>(
    &self,
    key: &str,
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
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(match opt {
      Some(opt) => cmd.set_options(key, now, opt)?,
      None => cmd.set(key, now)?,
    });
  }

  async fn del_last_checked<R>(&self, keys: &[Arc<str>]) -> KVSResult<R>
  where
    R: FromRedisValue + Send,
  {
    let keys: Vec<String> = keys
      .into_iter()
      .map(|key| self.get_timestamp_channel(key))
      .collect();
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.del(keys)?);
  }
}
