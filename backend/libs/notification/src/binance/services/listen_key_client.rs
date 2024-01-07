use ::std::time::Duration;

use ::async_trait::async_trait;
use ::url::Url;

use ::clients::binance::REST_ENDPOINTS;
use ::errors::NotificationResult;
use ::round_robin_client::RestClient;
use ::keychain::IQueryStringSigner

use super::super::interfaces::IListenKeyClient;

pub struct ListenKeyClient {
  cli: RestClient,
}

impl ListenKeyClient {
  pub fn new() -> NotificationResult<Self> {
    let url: NotificationResult<Vec<Url>> = REST_ENDPOINTS
      .iter()
      .map(|endpoint| {
        Ok(format!("{}/api/v3/userDataStream", endpoint).parse()?)
      })
      .collect();
    let url = url?;
    let cli =
      RestClient::new(&url, Duration::from_secs(5), Duration::from_secs(5))?;
    return Ok(Self { cli });
  }
}

impl IListenKeyClient for ListenKeyClient {
  async fn create(&self, api_key: Arc<APIKey>) -> UserStreamResult<ListenKey> {

  }
  async fn delete(
    &self,
    api_key: Arc<APIKey>,
    listen_key: Arc<ListenKey>,
  ) -> UserStreamResult<()>;
  async fn extend_lifetime(
    &self,
    api_key: Arc<APIKey>,
    listen_key: Arc<APIKey>,
  ) -> UserStreamResult<()>;
}
