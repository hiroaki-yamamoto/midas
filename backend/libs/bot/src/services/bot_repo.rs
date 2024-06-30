use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt};
use ::mongodb::bson::{doc, oid::ObjectId, to_document};
use ::mongodb::options::{FindOneOptions, UpdateOptions};
use ::mongodb::results::UpdateResult;
use ::mongodb::{Collection, Database};

use ::errors::ObjectNotFound;

use crate::entities::Bot;
use crate::errors::{BotInfoError, BotInfoResult};
use crate::interfaces::IBotRepo;

#[derive(Debug, Clone)]
pub struct BotRepo {
  col: Collection<Bot>,
}

impl BotRepo {
  pub fn new(db: &Database) -> Self {
    return Self {
      col: db.collection("bot"),
    };
  }
}

#[async_trait]
impl IBotRepo for BotRepo {
  async fn summary_by_id(&self, id: ObjectId) -> BotInfoResult<Bot> {
    let doc = self
      .col
      .find_one(doc! {"_id": id})
      .with_options(
        FindOneOptions::builder()
          .projection(doc! {"cond_ts": 0})
          .build(),
      )
      .await?
      .ok_or(ObjectNotFound::new("Bot", Some(id.to_hex().as_str())))?;
    return Ok(doc);
  }
  async fn get_by_id(&self, id: ObjectId) -> BotInfoResult<Bot> {
    let doc = self
      .col
      .find_one(doc! {"_id": id})
      .await?
      .ok_or(ObjectNotFound::new("Bot", Some(id.to_hex().as_str())))?;
    return Ok(doc);
  }
  async fn save(&self, model: &Bot) -> BotInfoResult<UpdateResult> {
    let result = self
      .col
      .update_one(doc! {"_id": model.id}, doc! {"$set": to_document(model)?})
      .with_options(UpdateOptions::builder().upsert(true).build())
      .await;
    return Ok(result?);
  }
  async fn list(&self) -> BotInfoResult<BoxStream<BotInfoResult<Bot>>> {
    let cursor = self
      .col
      .find(doc! {})
      .projection(doc! {
        "cond_ts": 0,
      })
      .await?
      .map(|doc_result| {
        return doc_result.map_err(|e| {
          return BotInfoError::from(e);
        });
      });
    return Ok(cursor.boxed());
  }
}
