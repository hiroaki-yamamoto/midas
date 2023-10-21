use ::std::time::{Duration, SystemTime};

use ::async_trait::async_trait;
use ::errors::KVSResult;

use crate::redis::{AsyncCommands as Commands, RedisResult};

use super::Base;

#[async_trait]
pub trait FindBefore<C>: Base<C>
where
  C: Commands + Send,
{
  async fn find_before(&self, dur: Duration) -> RedisResult<Vec<String>> {
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;

    let scan_pattern = self.get_timestamp_channel("*");
    let keys: Vec<String> =
      cmd.scan_match::<_, String>(scan_pattern)?.collect();
    let last_checked_timestamps: Vec<i64> = cmd.mget(&keys)?;
    let last_checked_timestamps: Vec<KVSResult<SystemTime>> =
      last_checked_timestamps
        .into_iter()
        .map(|timestamp| Self::convert_timestamp(timestamp))
        .collect();
    let keys: Vec<String> = keys
      .into_iter()
      .zip(last_checked_timestamps.into_iter())
      .filter_map(|(key, last_checked)| {
        if let Ok(last_checked) = last_checked {
          let threashold = SystemTime::now().checked_sub(dur);
          if let Some(threashold) = threashold {
            if last_checked < threashold {
              return Some(key);
            }
          }
        }
        return None;
      })
      .collect();

    return Ok(keys);
  }
}
