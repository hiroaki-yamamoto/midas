use ::mongodb::bson::{doc, from_document, to_document, Document};
use ::mongodb::error::Result as DBResult;
use ::mongodb::options::UpdateOptions;
use ::mongodb::{Collection, Database};

use ::types::ThreadSafeResult;

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

  pub async fn write(&self, model: &Bot) -> ThreadSafeResult<Bot> {
    let model_doc = to_document(&model)?;
    let id = match &model.id {
      Some(id) => self
        .col
        .update_one(
          doc! {"_id": id},
          model_doc,
          UpdateOptions::builder().upsert(true).build(),
        )
        .await?
        .upserted_id
        .map(|id_son| id_son.as_object_id().cloned())
        .flatten(),
      None => {
        let result = self.col.insert_one(model_doc, None).await?.inserted_id;
        result.as_object_id().cloned()
      }
    };
    let mut model = model.clone();
    if id != None {
      model.id = id;
    }
    return Ok(model);
  }
}
