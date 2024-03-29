use ::std::sync::Arc;
use ::std::time::SystemTime;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Local, LocalResult, TimeZone};
use ::errors::{KVSError, KVSResult};
use ::redis::{AsyncCommands as Commands, SetOptions};

use crate::options::WriteOption;
use crate::traits::base::{Base as BaseBase, ChannelName};

#[async_trait]
pub trait Base: BaseBase + ChannelName {
  fn get_timestamp_channel(&self, key: Arc<String>) -> String {
    return format!("last_check_timestamp:{}", self.__channel_name__(key));
  }

  fn convert_timestamp(&self, timestamp: i64) -> KVSResult<SystemTime> {
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

  async fn get_last_checked(&self, key: Arc<String>) -> KVSResult<SystemTime> {
    let key = self.get_timestamp_channel(key);
    let mut cmd = self.__commands__();
    // let mut cmd = cmd.lock().await;
    let timestamp: i64 = cmd.get(key).await?;
    return Ok(self.convert_timestamp(timestamp)?);
  }

  async fn flag_last_checked(
    &self,
    key: Arc<String>,
    opt: Option<WriteOption>,
  ) -> KVSResult<bool> {
    let key = self.get_timestamp_channel(key);
    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_secs();
    let opt: Option<SetOptions> = opt.map(|wo| wo.into());
    let mut cmd = self.__commands__();
    // let mut cmd = cmd.lock().await;
    return Ok(match opt {
      Some(opt) => cmd.set_options(key, now, opt).await?,
      None => cmd.set(key, now).await?,
    });
  }

  async fn del_last_checked(
    &self,
    keys: Arc<[Arc<String>]>,
  ) -> KVSResult<usize> {
    let keys: Vec<String> = keys
      .into_iter()
      .map(|key| self.get_timestamp_channel(key.clone()))
      .collect();
    let mut cmd = self.__commands__();
    // let mut cmd = cmd.lock().await;
    return Ok(cmd.del(keys).await?);
  }
}
