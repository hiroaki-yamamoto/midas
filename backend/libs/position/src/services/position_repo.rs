use ::async_trait::async_trait;
use ::mongodb::bson::{doc, oid::ObjectId, to_document};
use ::mongodb::options::UpdateOptions;
use ::mongodb::results::UpdateResult;
use ::mongodb::{Collection, Database};

use ::errors::{ObjectNotFound, PositionResult};
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
  async fn save(&self, position: &[&Position]) -> PositionResult<UpdateResult> {
    let ids: Vec<ObjectId> = position.iter().map(|p| p.id.clone()).collect();
    return Ok(
      self
        .col
        .update_many(
          doc! {
            "_id": { "$in": ids }
          },
          to_document(position)?,
          UpdateOptions::builder().upsert(true).build(),
        )
        .await?,
    );
  }

  async fn get(&self, id: &ObjectId) -> PositionResult<Position> {
    let position = self
      .col
      .find_one(doc! { "_id": id }, None)
      .await?
      .ok_or(ObjectNotFound::new("Position", id.to_hex().as_str()))?;
    return Ok(position);
  }
}
