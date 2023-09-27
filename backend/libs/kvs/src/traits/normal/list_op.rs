use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};
use ::std::num::NonZeroUsize;

use ::errors::{KVSError, KVSResult};

use super::{Base, Exist, Lock};

use crate::options::WriteOptionTrait;
use crate::WriteOption;

#[async_trait]
pub trait ListOp<T, V>: Base<T> + Lock<T> + Exist<T>
where
  T: Commands + Send,
  for<'a> V: FromRedisValue + ToRedisArgs + Send + 'a,
{
  async fn lpush<R>(
    &self,
    key: &str,
    value: Vec<V>,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(&key);
    let opt: Option<WriteOption> = opt.into();

    let cmds = self.commands();
    let mut cmds = cmds.lock().await;
    let res = if opt.non_existent_only() {
      match self.exists(&key).await {
        Ok(exists) => {
          if exists {
            return Err(KVSError::KeyExists(key.to_string()));
          } else {
            cmds.lpush(&channel_name, value)?
          }
        }
        Err(e) => return Err(e),
      }
    } else {
      cmds.lpush(&channel_name, value)?
    };

    opt.execute(&mut cmds, &channel_name)?;
    return Ok(res);
  }

  async fn lpop(&self, key: &str, count: Option<NonZeroUsize>) -> KVSResult<V> {
    let channel_name = self.channel_name(key);
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.lpop(channel_name, count)?);
  }

  async fn lrem<R>(&self, key: &str, count: isize, elem: V) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.lrem(channel_name, count, elem)?);
  }

  async fn lrange<R>(
    &self,
    key: &str,
    start: isize,
    stop: isize,
  ) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.lrange(channel_name, start, stop)?);
  }

  async fn llen<R>(&self, key: &str) -> KVSResult<R>
  where
    R: FromRedisValue,
  {
    let channel_name = self.channel_name(key);
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.llen(channel_name)?);
  }
}
