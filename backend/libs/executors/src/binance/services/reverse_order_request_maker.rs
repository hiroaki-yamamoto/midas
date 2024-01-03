use ::errors::{ExecutionErrors, ExecutionResult};
use ::keychain::APIKey;
use ::mongodb::bson::DateTime;
use ::rpc::exchanges::Exchanges;
use ::rug::{float::Round, Float};
use ::serde_qs::to_string as to_qs;

use ::position::binance::entities::{OrderResponse, OrderType, Side};

use super::super::{
  entities::OrderRequest, interfaces::IReverseOrderRequestMaker,
};

pub struct RequestMaker;

impl RequestMaker {
  pub fn new() -> Self {
    return Self;
  }
}

impl IReverseOrderRequestMaker for RequestMaker {
  fn build(
    &self,
    api_key: &APIKey,
    order: &OrderResponse<rug::Float, DateTime>,
  ) -> ExecutionResult<String> {
    let sum_qty = order.sum_filled_qty();
    let qty = Float::with_val(128, &sum_qty * 10000);
    let qty = qty
      .to_integer_round(Round::Down)
      .ok_or(ExecutionErrors::InvalidQty(sum_qty))?
      .0;
    let side = order.side.as_ref().unwrap_or(&Side::Buy).clone();
    let req =
      OrderRequest::<i64>::new(order.symbol.clone(), !side, OrderType::Market)
        .quantity(Some(qty.to_string()));
    let qs = to_qs(&req)?;
    let sign = api_key.sign(Exchanges::Binance, &qs);
    let qs = format!("{}&signature={}", qs, sign);
    return Ok(qs);
  }
}
