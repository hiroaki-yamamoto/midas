use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::errors::KVSResult;

use super::Expiration;

use crate::options::{WriteOption, WriteOptionTrait};

#[async_trait]
pub trait OptExecution: Expiration {
  async fn execute_opt(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
    opt: Option<WriteOption>,
  ) -> KVSResult<()> {
    if let Some(duration) = opt.duration() {
      self.expire(exchange, symbol, duration).await?;
    }
    return Ok(());
  }
}
