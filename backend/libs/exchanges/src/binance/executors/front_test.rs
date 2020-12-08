use ::std::collections::HashMap;

use ::mongodb::bson::oid::ObjectId;
use ::nats::asynk::Connection as NatsCon;
use ::slog::Logger;

use crate::traits::Executor as ExecutorTrait;

use crate::entities::BookTicker;

use super::super::TradeObserver;
use super::entities::{Order, OrderInner};

pub struct Executor {
  observer: TradeObserver,
  orders: HashMap<ObjectId, Order>,
  positions: HashMap<ObjectId, OrderInner>,
  cur_trade: Option<BookTicker>,
  maker_fee: f64,
  taker_fee: f64,
}

impl Executor {
  pub async fn new(
    logger: Logger,
    broker: NatsCon,
    maker_fee: f64,
    taker_fee: f64,
  ) -> Self {
    return Self {
      observer: TradeObserver::new(None, broker, logger).await,
      orders: HashMap::new(),
      positions: HashMap::new(),
      cur_trade: None,
      maker_fee,
      taker_fee,
    };
  }
}
