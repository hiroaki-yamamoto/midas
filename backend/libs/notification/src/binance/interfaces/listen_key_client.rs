use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::errors::UserStreamResult;
use ::keychain::APIKey;

use super::super::entities::ListenKey;

#[async_trait]
pub trait IListenKeyClient {
  async fn create(&self, api_key: Arc<APIKey>) -> UserStreamResult<ListenKey>;
  async fn delete(
    &self,
    api_key: Arc<APIKey>,
    listen_key: Arc<ListenKey>,
  ) -> UserStreamResult<()>;
  async fn extend_lifetime(
    &self,
    api_key: Arc<APIKey>,
    listen_key: Arc<ListenKey>,
  ) -> UserStreamResult<()>;
}
