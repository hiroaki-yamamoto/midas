use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::redis::{AsyncCommands as Commands, FromRedisValue, ToRedisArgs};
use ::std::num::NonZeroUsize;

use ::errors::{KVSError, KVSResult};

use super::{Base, Exist};

use crate::options::WriteOptionTrait;
use crate::WriteOption;

#[async_trait]
pub trait ListOp {
  async fn __lpush__(
    &self,
    key: Arc<String>,
    value: Vec<Arc<dyn FromRedisValue>>,
    opt: impl Into<Option<WriteOption>> + Send,
  ) -> KVSResult<usize> {
    let channel_name = self.__channel_name__(key.clone());
    let opt: Option<WriteOption> = opt.into();

    let mut cmds = self.__commands__();
    let res = if opt.non_existent_only() {
      match self.__exists__(key.clone()).await {
        Ok(exists) => {
          if exists {
            return Err(KVSError::KeyExists(key.to_string()));
          } else {
            // let mut cmds = cmds.lock().await;
            cmds.lpush(channel_name.as_ref(), value).await
          }
        }
        Err(e) => return Err(e),
      }
    } else {
      // let mut cmds = cmds.lock().await;
      cmds.lpush(channel_name.as_ref(), value).await
    }?;

    opt.execute(cmds, channel_name).await?;
    return Ok(res);
  }

  async fn __lpop__(
    &self,
    key: Arc<String>,
    count: Option<NonZeroUsize>,
  ) -> KVSResult<Arc<dyn FromRedisValue>> {
    let channel_name = self.__channel_name__(key);
    let mut cmd = self.__commands__();
    return Ok(cmd.lpop(channel_name.as_ref(), count).await?);
  }

  async fn __lrem__(
    &self,
    key: Arc<String>,
    count: isize,
    elem: Arc<dyn ToRedisArgs>,
  ) -> KVSResult<usize> {
    let channel_name = self.__channel_name__(key);
    let mut cmd = self.__commands__();
    return Ok(cmd.lrem(channel_name.as_ref(), count, elem).await?);
  }

  async fn __lrange__(
    &self,
    key: Arc<String>,
    start: isize,
    stop: isize,
  ) -> KVSResult<Vec<Arc<dyn FromRedisValue>>> {
    let channel_name = self.__channel_name__(key);
    let mut cmd = self.__commands__();
    return Ok(cmd.lrange(channel_name.as_ref(), start, stop).await?);
  }

  async fn __llen__(&self, key: Arc<String>) -> KVSResult<usize> {
    let channel_name = self.__channel_name__(key);
    let mut cmd = self.__commands__();
    return Ok(cmd.llen(channel_name.as_ref()).await?);
  }
}
