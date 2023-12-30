use ::async_trait::async_trait;
use ::rug::Float;

use ::entities::OrderOption;
use ::errors::ExecutionResult;
use ::keychain::APIKey;

#[async_trait]
pub trait INewOrderRequestMaker {
  async fn build(
    &self,
    api_key: &APIKey,
    symbol: String,
    budget: Float,
    price: Option<Float>,
    order_option: Option<OrderOption>,
  ) -> ExecutionResult<Vec<String>>;
}
