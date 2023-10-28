use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::base::Expiration as BaseExp;

use super::base::Base;

#[async_trait]
pub trait Expiration: Base + BaseExp {
  async fn expire(&self, key: Arc<String>, dur: Duration) -> KVSResult<bool> {
    let ret = self.__expire__(key.clone(), dur).await?;
    let opt: WriteOption = WriteOption::default().duration(dur.into());
    self.flag_last_checked(key, opt.into()).await?;
    return Ok(ret);
  }
}
