use ::std::time::Duration;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::normal::Expiration as NormalExp;

use super::base::Base;

#[async_trait]
pub trait Expiration<T, V>: Base<T, V> + NormalExp<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
  async fn expire(&self, key: &str, dur: Duration) -> KVSResult<bool> {
    let ret = NormalExp::expire(self, key, dur).await?;
    self
      .flag_last_checked(
        key,
        WriteOption::default().duration(dur.into()).into(),
      )
      .await?;
    return Ok(ret);
  }
}
