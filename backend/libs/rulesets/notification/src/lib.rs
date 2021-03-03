use ::async_trait::async_trait;

use ::entities::APIKeyInner;
use ::types::GenericResult;

#[async_trait]
pub trait UserStream {
  async fn get_listen_key(&self, api_key: &APIKeyInner) -> GenericResult<()>;
  async fn clise_listen_key(
    &self,
    api_key: &APIKeyInner,
    listen_key: &String,
  ) -> GenericResult<()>;
  async fn start(&self) -> GenericResult<()>;
}
