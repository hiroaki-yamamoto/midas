use ::keychain::APIKey;
use ::mongodb::bson::DateTime;
use ::rug::Float;

use ::errors::ExecutionResult;
use ::position::binance::entities::OrderResponse;

pub trait IReverseOrderRequestMaker {
  fn build(
    &self,
    api_key: &APIKey,
    order: &OrderResponse<Float, DateTime>,
  ) -> ExecutionResult<String>;
}
