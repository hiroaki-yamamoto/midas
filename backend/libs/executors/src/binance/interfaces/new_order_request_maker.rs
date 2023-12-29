use ::async_trait::async_trait;
use ::mongodb::bson::oid::ObjectId;
use ::rug::Float;

use ::entities::OrderOption;
use ::errors::ExecutionResult;

#[async_trait]
pub trait INewOrderRequestMaker {
  async fn build(
    &self,
    api_key_id: ObjectId,
    symbol: String,
    budget: Float,
    price: Option<Float>,
    order_option: Option<OrderOption>,
  ) -> ExecutionResult<Vec<String>>;
}
