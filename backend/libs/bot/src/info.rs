use ::rpc::bot::Bot as RPCBot;

use ::mongodb::bson::{doc, from_document, Document};
use ::mongodb::error::Result as DBResult;
use ::mongodb::options::UpdateModifications;
use ::mongodb::{Collection, Database};

use super::entities::Bot;

pub struct BotInfoRecorder {
  col: Collection,
}

impl BotInfoRecorder {
  pub fn new(db: &Database) -> Self {
    return Self {
      col: db.collection("bot"),
    };
  }

  pub async fn get(
    &self,
    query: impl Into<Option<Document>>,
  ) -> DBResult<Option<Bot>> {
    let doc: Option<Bot> = self
      .col
      .find_one(query, None)
      .await?
      .map(|doc| from_document(doc).ok())
      .flatten();
    return Ok(doc);
  }

  pub async fn write(&self, model: Bot) -> Result<Bot> {
    // self.col.update_one(doc! {"_id": model.id})
  }
}
