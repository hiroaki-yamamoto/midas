use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, FromRedisValue, ToRedisArgs};
use ::std::num::NonZeroUsize;

use ::errors::{KVSError, KVSResult};

use super::{Base, Exist};

use crate::options::WriteOptionTrait;
use crate::WriteOption;

#[async_trait]
pub trait ListOp<T, V>: Base<T> + Exist<T>
where
  T: Commands + Send,
  for<'a> V: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
{
  async fn lpush(
    &self,
    key: &str,
    value: Vec<V>,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<usize> {
    let channel_name = self.channel_name(&key);
    let opt: Option<WriteOption> = opt.into();

    let mut cmds = self.commands();
    let res = if opt.non_existent_only() {
      match self.exists(&key).await {
        Ok(exists) => {
          if exists {
            return Err(KVSError::KeyExists(key.to_string()));
          } else {
            // let mut cmds = cmds.lock().await;
            cmds.lpush(&channel_name, value).await
          }
        }
        Err(e) => return Err(e),
      }
    } else {
      // let mut cmds = cmds.lock().await;
      cmds.lpush(&channel_name, value).await
    }?;

    opt.execute(cmds, &channel_name).await?;
    return Ok(res);
  }

  async fn lpop(&self, key: &str, count: Option<NonZeroUsize>) -> KVSResult<V> {
    let channel_name = self.channel_name(key);
    let mut cmd = self.commands();
    // let mut cmd = cmd.lock().await;
    return Ok(cmd.lpop(channel_name, count).await?);
  }

  async fn lrem(&self, key: &str, count: isize, elem: V) -> KVSResult<usize> {
    let channel_name = self.channel_name(key);
    let mut cmd = self.commands();
    // let mut cmd = cmd.lock().await;
    return Ok(cmd.lrem(channel_name, count, elem).await?);
  }

  async fn lrange(
    &self,
    key: &str,
    start: isize,
    stop: isize,
  ) -> KVSResult<Vec<V>> {
    let channel_name = self.channel_name(key);
    let mut cmd = self.commands();
    // let mut cmd = cmd.lock().await;
    return Ok(cmd.lrange(channel_name, start, stop).await?);
  }

  async fn llen(&self, key: &str) -> KVSResult<usize> {
    let channel_name = self.channel_name(key);
    let mut cmd = self.commands();
    // let mut cmd = cmd.lock().await;
    return Ok(cmd.llen(channel_name).await?);
  }
}
