use ::rug::Float;

use ::entities::OrderOption;

use crate::binance::entities::OrderRequest;

pub trait INewOrderRequestMaker {
  fn build(
    &self,
    symbol: String,
    budget: Float,
    price: Option<Float>,
    order_option: Option<OrderOption>,
  ) -> Vec<OrderRequest<i64>>;
}
