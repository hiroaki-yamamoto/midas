use ::std::sync::Arc;

use ::rug::Float;
use ::serde_qs::{to_string as to_qs, Error as QsErr};

use ::entities::OrderOption;
use ::errors::ExecutionResult;
use ::sign::ISigner;

use super::super::{
  entities::{OrderRequest, OrderResponseType, OrderType, Side},
  interfaces::INewOrderRequestMaker,
};

pub struct RequestMaker {
  signer: Arc<dyn ISigner + Send + Sync>,
}

impl RequestMaker {
  pub fn new(signer: Arc<dyn ISigner + Send + Sync>) -> Self {
    Self { signer }
  }

  fn create_order_requests(
    &self,
    symbol: String,
    budget: Float,
    price: Option<Float>,
    order_option: OrderOption,
    order_type: OrderType,
  ) -> Vec<OrderRequest<i64>> {
    let req = order_option
      .calc_trading_amounts(budget)
      .into_iter()
      .enumerate()
      .map(move |(index, tr_amount)| {
        let mut order = OrderRequest::<i64>::new(
          symbol.clone(),
          Side::Buy,
          order_type.clone(),
        );
        if order_option.iceberg {
          order = order.iceberg_qty(Some(tr_amount.to_string()));
        } else {
          order = order.quantity(Some(tr_amount.to_string()));
        }
        if order_type == OrderType::Limit {
          order = order.price(Some(
            order_option
              .calc_order_price(price.clone().unwrap(), index)
              .to_string(),
          ));
        }
        order = order.order_response_type(Some(OrderResponseType::RESULT));
        return order;
      });
    return req.collect();
  }
}

impl INewOrderRequestMaker for RequestMaker {
  fn build(
    &self,
    symbol: String,
    budget: Float,
    price: Option<Float>,
    order_option: Option<OrderOption>,
  ) -> ExecutionResult<Vec<String>> {
    let order_type =
      price.map(|_| OrderType::Limit).unwrap_or(OrderType::Market);
    let req = order_option
      .map(|o| self.create_order_requests(symbol, budget, price, o, order_type))
      .unwrap_or_else(|| {
        let mut order =
          OrderRequest::<i64>::new(symbol, Side::Buy, order_type.clone())
            .order_response_type(Some(OrderResponseType::RESULT));
        if order_type == OrderType::Limit {
          order = order.price(price.map(|p| p.to_string()));
        }
        return vec![order];
      });
    let req: Result<Vec<String>, QsErr> = req
      .iter()
      .map(|order| {
        let qs = to_qs(order)?;
        let sign = self.signer.sign(qs);
        return Ok(format!("{}&signature={}", qs, sign));
      })
      .collect();
    return Ok(req?);
  }
}
