use ::std::sync::Arc;
use ::std::time::Duration as StdDur;

use ::async_trait::async_trait;
use ::mongodb::bson::DateTime;
use ::reqwest::header::HeaderMap;
use ::reqwest::Response;
use ::rug::Float;
use ::serde_qs::to_string as to_qs;

use ::clients::binance::REST_ENDPOINTS;
use ::errors::{ExecutionResult, StatusFailure};
use ::keychain::{APIKey, IHeaderSigner, IQueryStringSigner};
use ::position::binance::entities::OrderResponse;
use ::round_robin_client::RestClient;
use ::rpc::exchanges::Exchanges;

use super::super::{entities::OrderRequest, interfaces::IOrderClient};

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
    let client = RestClient::new(
      REST_ENDPOINTS
        .into_iter()
        .filter_map(|&url| format!("{}/api/v3/order", url).parse().ok())
        .collect(),
      StdDur::from_secs(5),
      StdDur::from_secs(5),
    )?;
    return Ok(Self {
      client: Arc::new(client),
      qs_signer,
      header_signer,
    });
  }

  async fn check_resp_status(&self, resp: &Response) -> ExecutionResult<()> {
    if resp.status().is_success() {
      return Ok(());
    }
    let status = resp.status();
    let text = resp
      .text()
      .await
      .unwrap_or(status.canonical_reason().unwrap_or("Unknown").to_string());
    return Err(
      StatusFailure::new(Some(resp.url().to_string()), status.as_u16(), text)
        .into(),
    );
  }
}

#[async_trait]
impl IOrderClient for Client {
  async fn new_order(
    &self,
    api_key: &APIKey,
    position: &OrderRequest<i64>,
  ) -> ExecutionResult<OrderResponse<Float, DateTime>> {
    let qs = self
      .qs_signer
      .append_sign(api_key, to_qs(position)?.as_str());
    let mut header = HeaderMap::default();
    self.header_signer.append_sign(api_key, &mut header)?;
    let resp = self.client.post(Some(header), Some(qs)).await?;
    self.check_resp_status(&resp).await?;
    let payload: OrderResponse<String, i64> = resp.json().await?;
    let payload = OrderResponse::<Float, DateTime>::try_from(payload)?;
    return Ok(payload);
  }
}
