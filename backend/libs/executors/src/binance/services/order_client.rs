use ::std::sync::Arc;
use ::std::time::Duration as StdDur;

use ::async_trait::async_trait;
use ::mongodb::bson::DateTime;
use ::reqwest::header::HeaderMap;
use ::reqwest::Response;
use ::rug::Float;
use ::serde_qs::to_string as to_qs;
use ::url::Url;

use ::clients::binance::REST_ENDPOINTS;
use ::errors::{ExecutionResult, StatusFailure};
use ::keychain::{APIKey, IHeaderSigner, IQueryStringSigner};
use ::position::binance::entities::OrderResponse;
use ::round_robin_client::RestClient;

use super::super::{
  entities::{CancelOrderRequest, OrderRequest},
  interfaces::IOrderClient,
};

pub struct Client {
  client: Arc<RestClient>,
  qs_signer: Arc<dyn IQueryStringSigner + Send + Sync>,
  header_signer: Arc<dyn IHeaderSigner + Send + Sync>,
}

impl Client {
  pub fn new(
    qs_signer: Arc<dyn IQueryStringSigner + Send + Sync>,
    header_signer: Arc<dyn IHeaderSigner + Send + Sync>,
  ) -> ExecutionResult<Self> {
    let url: ExecutionResult<Vec<Url>> = REST_ENDPOINTS
      .iter()
      .map(|url| Ok(format!("{}/api/v3/order", url).parse()?))
      .collect();
    let url = url?;
    let client =
      RestClient::new(&url, StdDur::from_secs(5), StdDur::from_secs(5))?;
    return Ok(Self {
      client: Arc::new(client),
      qs_signer,
      header_signer,
    });
  }

  async fn status_failure(&self, resp: Response) -> StatusFailure {
    let status = resp.status();
    let url = resp.url().to_string();
    let text = resp
      .text()
      .await
      .unwrap_or(status.canonical_reason().unwrap_or("Unknown").to_string());
    return StatusFailure::new(Some(url), status.as_u16(), text);
  }
}

#[async_trait]
impl IOrderClient for Client {
  async fn new_order(
    &self,
    api_key: Arc<APIKey>,
    position: Arc<OrderRequest<i64>>,
  ) -> ExecutionResult<OrderResponse<Float, DateTime>> {
    let api_key = api_key.as_ref();
    let qs = self
      .qs_signer
      .append_sign(api_key, to_qs(&position)?.as_str());
    let mut header = HeaderMap::default();
    self.header_signer.append_sign(api_key, &mut header)?;
    let resp = self.client.post(Some(header), Some(qs)).await?;
    if !resp.status().is_success() {
      return Err(self.status_failure(resp).await.into());
    }
    let payload: OrderResponse<String, i64> = resp.json().await?;
    let payload = OrderResponse::<Float, DateTime>::try_from(payload)?;
    return Ok(payload);
  }

  async fn cancel_order(
    &self,
    api_key: Arc<APIKey>,
    req: Arc<CancelOrderRequest<i64>>,
  ) -> ExecutionResult<OrderResponse<Float, DateTime>> {
    let api_key = api_key.as_ref();
    let qs = self.qs_signer.append_sign(api_key, to_qs(&req)?.as_str());
    let mut header = HeaderMap::default();
    self.header_signer.append_sign(api_key, &mut header)?;
    let resp = self.client.delete(Some(header), Some(qs)).await?;
    if !resp.status().is_success() {
      return Err(self.status_failure(resp).await.into());
    }
    let payload: OrderResponse<String, i64> = resp.json().await?;
    let payload = OrderResponse::<Float, DateTime>::try_from(payload)?;
    return Ok(payload);
  }
}
