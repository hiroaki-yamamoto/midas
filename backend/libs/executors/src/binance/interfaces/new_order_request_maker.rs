use ::rug::Float;

use ::entities::OrderOption;
use ::errors::ExecutionResult;

pub trait INewOrderRequestMaker {
  fn build(
    &self,
    symbol: String,
    budget: Float,
    price: Option<Float>,
    order_option: Option<OrderOption>,
  ) -> ExecutionResult<Vec<String>>;
}
