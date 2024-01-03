use ::rug::Float;

use ::errors::ExecutionResult;
use ::keychain::APIKey;
use ::mongodb::bson::DateTime;
use ::position::binance::entities::OrderResponse;

pub trait ICancelOrderRequestMaker {
  fn build(
    &self,
    api_key: &APIKey,
    order: &OrderResponse<Float, DateTime>,
  ) -> ExecutionResult<String>;
}
