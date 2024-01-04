use ::std::sync::Arc;
use ::std::time::Duration as StdDur;

use ::async_trait::async_trait;
use ::mongodb::bson::DateTime;
use ::rug::Float;

use ::clients::binance::{APIHeader, REST_ENDPOINTS};
use ::errors::ExecutionResult;
use ::keychain::APIKey;
use ::position::binance::entities::OrderResponse;
use ::round_robin_client::RestClient;

use super::super::{entities::OrderRequest, interfaces::IOrderClient};

pub struct Client {
  client: Arc<RestClient>,
}

impl Client {
  pub fn new() -> ExecutionResult<Self> {
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
    });
  }
}

impl APIHeader for Client {}

#[async_trait]
impl IOrderClient for Client {
  async fn new_order(
    &self,
    api_key: &APIKey,
    position: &OrderRequest<i64>,
  ) -> ExecutionResult<OrderResponse<Float, DateTime>> {
  }
}
