use ::mongodb::bson::oid::ObjectId;
use ::rug::Float;

use ::entities::OrderOption;
use ::errors::ExecutionResult;

pub trait INewOrderRequestMaker {
  fn build(
    &self,
    api_key_id: ObjectId,
    symbol: String,
    budget: Float,
    price: Option<Float>,
    order_option: Option<OrderOption>,
  ) -> ExecutionResult<Vec<String>>;
}
