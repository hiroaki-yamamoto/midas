use ::async_trait::async_trait;

use ::entities::APIKeyInner;
use ::types::{GenericResult, ThreadSafeResult};

#[async_trait]
pub trait UserStream {
  async fn get_listen_key(&self, api_key: &APIKeyInner)
    -> ThreadSafeResult<()>;
  async fn close_listen_key(
    &self,
    api_key: &APIKeyInner,
    listen_key: &String,
  ) -> ThreadSafeResult<()>;
  async fn start(&self) -> GenericResult<()>;
}
