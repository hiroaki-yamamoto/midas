use ::std::time::Duration;

use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::{doc, from_document, to_document, Document};
use ::mongodb::error::Result as MongoResult;
use ::mongodb::options::{CountOptions, UpdateOptions};
use ::mongodb::{Collection, Database};
use ::tokio::time::interval;

use ::types::{ret_on_err, GenericResult, SendableErrorResult};

use crate::entities::APIKey;
use crate::traits::Recorder;

#[derive(Debug, Clone)]
pub struct KeyChain {
  db: Database,
  col: Collection,
}

impl KeyChain {
  pub async fn new(db: Database) -> Self {
    let col = db.collection("apiKeyChains");
    let ret = Self { db, col };
    ret.update_indices(&["exchange"]).await;
    return ret;
  }

  pub async fn push(&self, mut value: APIKey) -> GenericResult<()> {
    value.id = self.gen_uid(&value.id).await?;
    let id = value.id.clone();
    let value = to_document(&value)?;
    let _ = self
      .col
      .update_one(
        doc! {"_id": id},
        value.to_owned(),
        UpdateOptions::builder().upsert(true).build(),
      )
      .await?;
    return Ok(());
  }

  pub async fn gen_uid(&self, id: &ObjectId) -> MongoResult<ObjectId> {
    let mut id = id.clone();
    let mut delay_ticker = interval(Duration::from_millis(100));
    for _ in 0..10 {
      let num_docs = self
        .col
        .count_documents(
          doc! {"_id": &id},
          CountOptions::builder().limit(1).build(),
        )
        .await?;
      if num_docs < 1 {
        break;
      }
      id = ObjectId::new();
      delay_ticker.tick().await;
    }
    return Ok(id);
  }

  pub async fn rename_label(
    &self,
    id: ObjectId,
    label: &str,
  ) -> GenericResult<()> {
    let _ = self
      .col
      .update_one(doc! { "_id": id }, doc! { "label": label }, None)
      .await?;
    return Ok(());
  }

  pub async fn list(
    &self,
    filter: Document,
  ) -> SendableErrorResult<BoxStream<'_, APIKey>> {
    let stream = ret_on_err!(self.col.find(filter, None).await)
      .filter_map(|res| async { res.ok() })
      .map(|doc| from_document::<APIKey>(doc))
      .filter_map(|ent| async { ent.ok() })
      .boxed();
    return Ok(stream);
  }

  pub async fn delete(&self, query: Document) -> GenericResult<()> {
    self.col.delete_many(query, None).await?;
    return Ok(());
  }
}

impl Recorder for KeyChain {
  fn get_database(&self) -> &Database {
    return &self.db;
  }

  fn get_col_name(&self) -> &str {
    return self.col.name();
  }
}
