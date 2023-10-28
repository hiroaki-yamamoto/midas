use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::errors::KVSResult;

use super::expiration::Expiration;

use crate::options::{WriteOption, WriteOptionTrait};

#[async_trait]
pub trait OptExecution: Expiration {
  async fn __execute_opt__(
    &self,
    key: Arc<String>,
    opt: Option<WriteOption>,
  ) -> KVSResult<()> {
    let mut res: KVSResult<()> = Ok(());
    if let Some(duration) = opt.duration() {
      self.__expire__(key, duration).await?;
    }
    return res;
  }
}
