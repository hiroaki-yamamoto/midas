use ::rug::Float;

use ::entities::OrderOption;
use ::errors::ExecutionResult;
use ::keychain::APIKey;

pub trait INewOrderRequestMaker {
  fn build(
    &self,
    api_key: &APIKey,
    symbol: String,
    budget: Float,
    price: Option<Float>,
    order_option: Option<OrderOption>,
  ) -> ExecutionResult<Vec<String>>;
}
