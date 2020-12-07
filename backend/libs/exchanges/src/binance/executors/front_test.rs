use ::std::collections::HashMap;

use ::mongodb::bson::oid::ObjectId;
use ::nats::asynk::Connection as NatsCon;

use crate::traits::Executor as ExecutorTrait;

use super::entities::{Order, OrderInner};

pub struct Executor {
  broker: NatsCon,
  orders: HashMap<ObjectId, Order>,
  positions: HashMap<ObjectId, OrderInner>,
}

impl Executor {
  pub fn new(broker: NatsCon) -> Self {
    return Self {
      broker,
      orders: HashMap::new(),
      positions: HashMap::new(),
    };
  }
}

// impl ExecutorTrait for Executor {
//   async fn create_order(
//     &mut self,
//     symbol: String,
//     price: Option<f64>,
//     budget: f64,
//     order_option: Option<OrderOption>,
//   ) -> GenericResult<ObjectId> {
//   }

//   async fn remove_order(
//     &mut self,
//     id: ObjectId,
//   ) -> GenericResult<ExecutionResult> {
//   }
// }
