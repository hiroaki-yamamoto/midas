use ::async_trait::async_trait;

use ::entities::APIKeyInner;
use ::errors::NotificationResult;

#[async_trait]
pub trait UserStream {
  async fn get_listen_key(
    &mut self,
    api_key: &APIKeyInner,
  ) -> NotificationResult<()>;
  async fn close_listen_key(
    &mut self,
    api_key: &APIKeyInner,
    listen_key: &String,
  ) -> NotificationResult<()>;
  async fn start(&mut self) -> NotificationResult<()>;
}
