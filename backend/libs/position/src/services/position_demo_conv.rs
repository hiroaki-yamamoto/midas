use ::async_trait::async_trait;
use ::mongodb::bson::oid::ObjectId;
use ::rand::{thread_rng, Rng};
use ::std::time::Duration;

use ::entities::OrderInner;
use ::errors::PositionResult;
use ::rpc::bot_mode::BotMode;
use ::rpc::position::Position as RPCPosition;
use ::rpc::position_status::PositionStatus;
use ::types::chrono::Utc;
use ::types::DateTime;

use crate::entities::Position;
use crate::interfaces::IPositionConverter;

pub struct PositionDemoConverter;

impl PositionDemoConverter {
  pub fn new() -> Self {
    return Self;
  }
}

#[async_trait]
impl IPositionConverter for PositionDemoConverter {
  async fn to_rpc(&self, position: &Position) -> PositionResult<RPCPosition> {
    let mut rng = thread_rng();
    let mut entries = Vec::<OrderInner>::new();
    let mut exits = Vec::<OrderInner>::new();
    for _ in 0..10 {
      entries.push(OrderInner::random());
      exits.push(OrderInner::random());
    }
    let entries_sum: OrderInner = entries.iter().sum();
    let exits_sum: OrderInner = exits.iter().sum();
    let now = Utc::now();
    let entry_at: DateTime = now - Duration::from_secs(604800); // 1 week ago
    let exit_at: DateTime = now.into();
    let is_closed: bool = rng.gen();
    return Ok(RPCPosition {
      id: ObjectId::new().to_hex(),
      symbol: position.symbol.clone(),
      amount: (entries_sum.qty * entries_sum.price.clone()).to_string(),
      bot_id: position.bot_id.to_hex(),
      mode: Box::new(BotMode::Live),
      entry_at: Box::new(entry_at.into()),
      entry_price: entries_sum.price.to_string(),
      exit_at: if is_closed {
        Some(Box::new(exit_at.into()))
      } else {
        None
      },
      exit_price: if is_closed {
        Some(exits_sum.price.to_string())
      } else {
        None
      },
      status: if is_closed {
        Box::new(PositionStatus::CLOSE)
      } else {
        Box::new(PositionStatus::OPEN)
      },
    });
  }
}
