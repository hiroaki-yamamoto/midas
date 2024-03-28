use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt, TryStreamExt};
use ::mongodb::bson::{doc, oid::ObjectId, to_document};
use ::mongodb::options::{FindOptions, UpdateOptions};
use ::mongodb::{Collection, Database};

use ::errors::{ObjectNotFound, PositionError, PositionResult};
use ::rpc::pagination::Pagination;
use ::writers::DatabaseWriter;

use crate::entities::Position;
use crate::interfaces::IPositionRepo;

pub struct PositionRepo {
  db: Database,
  col: Collection<Position>,
}

impl PositionRepo {
  pub async fn new(db: Database) -> Self {
    let col = db.collection("positions");
    let me = Self { db, col };
    let _ = me
      .update_indices(&["symbol", "bot_id", "entry_at", "exit_at"])
      .await;
    return me;
  }
}

impl DatabaseWriter for PositionRepo {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return self.col.name();
  }
}

#[async_trait]
impl IPositionRepo for PositionRepo {
  async fn save(&self, position: &Position) -> PositionResult<Position> {
    let result = self
      .col
      .update_one(
        doc! {"_id": &position.id},
        doc! { "$set": to_document(position)? },
        UpdateOptions::builder().upsert(true).build(),
      )
      .await?;
    let res_pos = if let Some(id) = result.upserted_id {
      let id = id.as_object_id().ok_or(PositionError::BSONCastFailed(id))?;
      Position {
        id,
        ..position.clone()
      }
    } else {
      position.clone()
    };
    return Ok(res_pos);
  }

  async fn get(&self, id: &ObjectId) -> PositionResult<Position> {
    let position = self
      .col
      .find_one(doc! { "_id": id }, None)
      .await?
      .ok_or(ObjectNotFound::new("Position", id.to_hex().as_str()))?;
    return Ok(position);
  }

  async fn list_by_bot_id(
    &self,
    bot_id: ObjectId,
    pg: Pagination,
  ) -> PositionResult<BoxStream<'_, PositionResult<Position>>> {
    let mut query = doc! { "bot_id": bot_id };
    if let Some(id) = pg.id {
      query.insert("_id", doc! { "$gt": id });
    }
    return Ok(
      self
        .col
        .find(
          query,
          FindOptions::builder()
            .limit(if pg.limit > 0 { Some(pg.limit) } else { None })
            .build(),
        )
        .await?
        .map_err(|e| {
          let e: PositionError = e.into();
          return e;
        })
        .boxed(),
    );
  }
}
