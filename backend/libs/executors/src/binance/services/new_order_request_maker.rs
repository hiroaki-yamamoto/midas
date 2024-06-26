use ::rug::Float;

use ::entities::OrderOption;
use ::position::binance::entities::{OrderType, Side};

use super::super::{
  entities::{OrderRequest, OrderResponseType},
  interfaces::INewOrderRequestMaker,
};

pub struct RequestMaker;

impl RequestMaker {
  pub fn new() -> Self {
    return Self {};
  }

  fn create_order_requests(
    &self,
    symbol: &str,
    budget: &Float,
    price: &Option<Float>,
    order_option: OrderOption,
    order_type: &OrderType,
  ) -> Vec<OrderRequest<i64>> {
    let req = order_option
      .calc_trading_amounts(budget)
      .into_iter()
      .enumerate()
      .map(move |(index, tr_amount)| {
        let mut order = OrderRequest::<i64>::new(
          symbol.to_string(),
          Side::Buy,
          order_type.clone(),
        );
        if order_option.iceberg {
          order = order.iceberg_qty(Some(tr_amount.to_string()));
        } else {
          order = order.quantity(Some(tr_amount.to_string()));
        }
        if let Some(price) = price {
          if order_type == &OrderType::Limit {
            order = order.price(Some(
              order_option.calc_order_price(price, index).to_string(),
            ));
          }
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
  ) -> Vec<OrderRequest<i64>> {
    let order_type = price
      .as_ref()
      .map(|_| OrderType::Limit)
      .unwrap_or(OrderType::Market);
    let req = order_option
      .map(|o| {
        let symbol = symbol.clone();
        self.create_order_requests(&symbol, &budget, &price, o, &order_type)
      })
      .unwrap_or_else(move || {
        let mut order =
          OrderRequest::<i64>::new(symbol, Side::Buy, order_type.clone())
            .order_response_type(Some(OrderResponseType::RESULT));
        if order_type == OrderType::Limit {
          order = order.price(price.as_ref().map(|p| p.to_string()));
        }
        return vec![order];
      });
    return req;
  }
}
