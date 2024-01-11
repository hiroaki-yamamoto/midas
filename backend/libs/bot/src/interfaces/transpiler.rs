use ::async_trait::async_trait;

use ::errors::HTTPResult;

use crate::entities::Bot;

#[async_trait]
pub trait ITranspiler {
  async fn transpile(&self, bot: Bot) -> HTTPResult<Bot>;
}
