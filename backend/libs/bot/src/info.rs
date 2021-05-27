use ::rpc::bot::Bot;

use ::mongodb::bson::{from_document, Document};
use ::mongodb::error::Result as DBResult;
use ::mongodb::{Collection, Database};

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
}
