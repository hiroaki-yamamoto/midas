use ::async_trait::async_trait;
use ::futures::{stream::BoxStream, StreamExt};
use ::mongodb::bson::oid::ObjectId;
use ::tokio_stream::iter;

use ::errors::PositionResult;
use ::rpc::bot_mode::BotMode;
use ::rpc::pagination::Pagination;

use crate::entities::Position;
use crate::interfaces::IPositionRepo;

pub struct PositionDemoRepo;

impl PositionDemoRepo {
  fn generate_oid_lt(&self, oid: &ObjectId) -> ObjectId {
    let mut ret = oid.clone();
    while &ret <= oid {
      ret = ObjectId::new();
    }
    return ret;
  }
}

#[async_trait]
impl IPositionRepo for PositionDemoRepo {
  async fn save(&self, position: &Position) -> PositionResult<Position> {
    return Ok(position.clone());
  }
  async fn get(&self, id: &ObjectId) -> PositionResult<Position> {
    let mut pos = Position::new(ObjectId::new(), BotMode::Live, "BTCUSD");
    pos.id = id.clone();
    return Ok(pos);
  }
  async fn list_by_bot_id(
    &self,
    bot_id: ObjectId,
    pg: Pagination,
  ) -> PositionResult<BoxStream<'_, PositionResult<Position>>> {
    let pg_id: Option<ObjectId> = pg.id.map(|id| id.parse().ok()).flatten();
    let mut pos: Vec<Position> = Vec::new();
    let set_id = |pos: &mut Position| {
      pos.id = if let Some(pg_id) = pg_id {
        self.generate_oid_lt(&pg_id)
      } else {
        ObjectId::new()
      }
    };
    for _ in 0..pg.limit {
      let mut item = Position::new(bot_id.clone(), BotMode::Live, "BTCUSD");
      set_id(&mut item);
      while pos.iter().find(|p| p.id == item.id).is_some() {
        set_id(&mut item);
      }
      pos.push(item);
    }
    return Ok(iter(pos).map(|pos| Ok(pos)).boxed());
  }
}
