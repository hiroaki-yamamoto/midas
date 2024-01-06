use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::mongodb::bson::DateTime;
use ::rug::Float;

use ::errors::ExecutionResult;
use ::keychain::APIKey;
use ::position::binance::entities::OrderResponse;

use super::super::entities::{CancelOrderRequest, OrderRequest};

#[async_trait]
pub trait IOrderClient {
  async fn new_order(
    &self,
    api_key: Arc<APIKey>,
    position: Arc<OrderRequest<i64>>,
  ) -> ExecutionResult<OrderResponse<Float, DateTime>>;
  async fn cancel_order(
    &self,
    api_key: Arc<APIKey>,
    req: Arc<CancelOrderRequest<i64>>,
  ) -> ExecutionResult<OrderResponse<Float, DateTime>>;
}
