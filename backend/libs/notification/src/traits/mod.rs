use ::async_trait::async_trait;

use ::errors::NotificationResult;

#[async_trait]
pub trait UserStream {
  async fn start(&self) -> NotificationResult<()>;
}
