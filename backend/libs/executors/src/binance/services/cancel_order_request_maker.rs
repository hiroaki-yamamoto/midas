use ::mongodb::bson::DateTime;
use ::rug::Float;
use ::serde_qs::to_string as to_qs;

use ::errors::{ExecutionErrors, ExecutionResult};
use ::keychain::APIKey;
use ::position::binance::entities::OrderResponse;
use ::rpc::exchanges::Exchanges;

use super::super::{
  entities::CancelOrderRequest, interfaces::ICancelOrderRequestMaker,
};

pub struct RequestMaker;

impl RequestMaker {
  pub fn new() -> Self {
    return Self {};
  }
}

impl ICancelOrderRequestMaker for RequestMaker {
  fn build(
    &self,
    api_key: &APIKey,
    order: &OrderResponse<Float, DateTime>,
  ) -> ExecutionResult<String> {
    if order.check_filled() {
      return Err(ExecutionErrors::OrderFilled);
    }
    let cancel_order = CancelOrderRequest::<i64>::new(order.symbol.clone())
      .order_id(Some(order.order_id));
    let mut qs = to_qs(&cancel_order)?;
    let sig = api_key.sign(Exchanges::Binance, &qs);
    qs = format!("{}&signature={}", qs, sig);
    return Ok(qs);
  }
}
