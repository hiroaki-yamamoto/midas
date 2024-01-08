use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::reqwest::header::HeaderMap;
use ::url::Url;

use ::clients::binance::REST_ENDPOINTS;
use ::errors::NotificationResult;
use ::errors::UserStreamResult;
use ::keychain::APIKey;
use ::keychain::IHeaderSigner;
use ::round_robin_client::RestClient;

use super::super::{entities::ListenKey, interfaces::IListenKeyClient};

pub struct ListenKeyClient {
  cli: RestClient,
  signer: Arc<dyn IHeaderSigner + Send + Sync>,
}

impl ListenKeyClient {
  pub fn new(
    signer: Arc<dyn IHeaderSigner + Send + Sync>,
  ) -> NotificationResult<Self> {
    let url: NotificationResult<Vec<Url>> = REST_ENDPOINTS
      .iter()
      .map(|endpoint| {
        Ok(format!("{}/api/v3/userDataStream", endpoint).parse()?)
      })
      .collect();
    let url = url?;
    let cli =
      RestClient::new(&url, Duration::from_secs(5), Duration::from_secs(5))?;
    return Ok(Self { cli, signer });
  }
}

#[async_trait]
impl IListenKeyClient for ListenKeyClient {
  async fn create(&self, api_key: Arc<APIKey>) -> UserStreamResult<ListenKey> {
    let mut header = HeaderMap::default();
    self.signer.append_sign(&api_key, &mut header)?;
    let resp: ListenKey = self
      .cli
      .post::<()>(Some(header), None)
      .await?
      .error_for_status()?
      .json()
      .await?;
    return Ok(resp);
  }
  async fn delete(
    &self,
    api_key: Arc<APIKey>,
    listen_key: Arc<ListenKey>,
  ) -> UserStreamResult<()> {
    let mut header = HeaderMap::default();
    self.signer.append_sign(&api_key, &mut header)?;
    let _ = self
      .cli
      .delete(Some(header), Some(listen_key))
      .await?
      .error_for_status()?;
    return Ok(());
  }
  async fn extend_lifetime(
    &self,
    api_key: Arc<APIKey>,
    listen_key: Arc<ListenKey>,
  ) -> UserStreamResult<()> {
    let mut header = HeaderMap::default();
    self.signer.append_sign(&api_key, &mut header)?;
    let _ = self
      .cli
      .put(Some(header), Some(listen_key))
      .await?
      .error_for_status()?;
    return Ok(());
  }
}
