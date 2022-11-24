use ::mongodb::bson::{doc, to_document, Document};
use ::mongodb::error::Result as DBResult;
use ::mongodb::options::UpdateOptions;
use ::mongodb::{Collection, Database};

use super::entities::Bot;
use super::errors::Result as BotInfoResult;

#[derive(Debug, Clone)]
pub struct BotInfoRecorder {
  col: Collection<Bot>,
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
    let doc: Option<Bot> = self.col.find_one(query, None).await?;
    return Ok(doc);
  }

  pub async fn write(&self, model: &Bot) -> BotInfoResult<Bot> {
    let id = match &model.id {
      Some(id) => self
        .col
        .update_one(
          doc! {"_id": id},
          to_document(&model)?,
          UpdateOptions::builder().upsert(true).build(),
        )
        .await?
        .upserted_id
        .map(|id_son| id_son.as_object_id())
        .flatten(),
      None => {
        let result = self.col.insert_one(model, None).await?.inserted_id;
        result.as_object_id()
      }
    };
    let mut model = model.clone();
    if id != None {
      model.id = id;
    }
    return Ok(model);
  }
}
