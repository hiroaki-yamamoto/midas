use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::{doc, from_document, to_document, Document};
use ::mongodb::{Collection, Database};

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

  pub async fn write(&self, mut value: APIKey) -> GenericResult<()> {
    value.id = self.generate_unique_id(ObjectId::new()).await;
    let id = value.id.clone();
    let value = to_document(&value)?;
    if let Err(_) = self
      .col
      .update_one(doc! {"_id": id}, value.to_owned(), None)
      .await
    {
      let _ = self.col.insert_one(value, None).await?;
    }
    return Ok(());
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

  async fn generate_unique_id(&self, id: ObjectId) -> ObjectId {
    let mut id = id;
    while let Ok(op) = self.col.find_one(doc! { "_id": &id }, None).await {
      match op {
        None => {
          break;
        }
        Some(_) => {
          id = ObjectId::new();
        }
      }
    }
    return id;
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
