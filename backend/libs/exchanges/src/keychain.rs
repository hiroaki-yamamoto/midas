use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::{doc, from_document, to_document, Document};
use ::mongodb::options::UpdateModifications;
use ::mongodb::{Collection, Database};

use ::rpc::entities::Exchanges;
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

  pub async fn push(&self, value: APIKey) -> GenericResult<Option<ObjectId>> {
    let value = to_document(&value)?;
    let result = self.col.insert_one(value.to_owned(), None).await?;
    let id = result.inserted_id.as_object_id();
    return Ok(id.cloned());
  }

  pub async fn rename_label(
    &self,
    id: ObjectId,
    label: &str,
  ) -> GenericResult<()> {
    let _ = self
      .col
      .update_one(
        doc! { "_id": id },
        UpdateModifications::Pipeline(vec![doc! {
          "$set": doc! {"label": label},
        }]),
        None,
      )
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

  pub async fn get(
    &self,
    exchange: Exchanges,
    id: ObjectId,
  ) -> GenericResult<Option<APIKey>> {
    let key = self
      .col
      .find_one(
        doc! {
          "_id": id,
          "exchange": exchange.as_string()
        },
        None,
      )
      .await?
      .map(|k| from_document::<APIKey>(k).ok())
      .flatten();
    return Ok(key);
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
