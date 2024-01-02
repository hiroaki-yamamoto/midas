use ::async_trait::async_trait;
use ::errors::PositionResult;
use ::futures::stream::{BoxStream, StreamExt, TryStreamExt};
use ::mongodb::bson::{doc, oid::ObjectId, to_document, DateTime};
use ::mongodb::options::UpdateOptions;
use ::mongodb::results::UpdateResult;
use ::mongodb::{Collection, Database};
use ::rug::Float;

use ::errors::PositionError;
use ::writers::DatabaseWriter;

use crate::entities::Position;

use super::super::{entities::OrderResponse, interfaces::IOrderResponseRepo};

pub struct OrderResponseRepo {
  db: Database,
  col: Collection<OrderResponse<Float, DateTime>>,
}

impl OrderResponseRepo {
  pub async fn new(db: Database) -> Self {
    let col = db.collection("binance.positions");
    let me = Self { db, col };
    let _ = me
      .update_indices(&[
        "symbol",
        "order_id",
        "client_order_id",
        "transact_time",
      ])
      .await;
    return me;
  }
}

impl DatabaseWriter for OrderResponseRepo {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return self.col.name();
  }
}

#[async_trait]
impl IOrderResponseRepo for OrderResponseRepo {
  async fn save(
    &self,
    order_response: &[&OrderResponse<Float, DateTime>],
  ) -> PositionResult<UpdateResult> {
    let ids: Vec<ObjectId> =
      order_response.iter().map(|p| p.id.clone()).collect();
    return Ok(
      self
        .col
        .update_many(
          doc! {
            "_id": { "$in": ids }
          },
          to_document(order_response)?,
          UpdateOptions::builder().upsert(true).build(),
        )
        .await?,
    );
  }

  async fn find_by_entry_position(
    &self,
    position: &Position,
  ) -> PositionResult<BoxStream<PositionResult<OrderResponse<Float, DateTime>>>>
  {
    let order_resp_stream = self
      .col
      .find(doc! {"gid": position.entry_gid}, None)
      .await?
      .map_err(|e| PositionError::from(e));
    return Ok(order_resp_stream.boxed());
  }
}
