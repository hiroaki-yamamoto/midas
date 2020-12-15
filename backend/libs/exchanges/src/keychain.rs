use ::mongodb::bson::{from_document, to_document, Document};
use ::mongodb::{Collection, Database};

use ::types::GenericResult;

use crate::entities::APIKey;
use crate::traits::Recorder;

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

  pub async fn write(&self, value: APIKey) -> GenericResult<()> {
    let value = to_document(&value)?;
    let _ = self.col.insert_one(value, None).await?;
    return Ok(());
  }

  pub async fn find(&self, filter: Document) -> GenericResult<Option<APIKey>> {
    let value = self
      .col
      .find_one(filter, None)
      .await?
      .map(|doc| from_document::<APIKey>(doc).ok())
      .flatten();
    return Ok(value);
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
