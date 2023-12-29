mod interfaces;
pub mod pubsub;

use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::{doc, Document};
use ::mongodb::error::Result;
use ::mongodb::options::UpdateModifications;
use ::mongodb::{Collection, Database};
use ::subscribe::nats::Client;
use ::subscribe::PubSub;

use ::errors::KeyChainResult;
use ::rpc::exchanges::Exchanges;

pub use crate::interfaces::IKeyChain;
pub use ::entities::APIKey;

use ::entities::APIKeyEvent;
use ::writers::DatabaseWriter as DBWriterTrait;

use self::pubsub::APIKeyPubSub;

#[derive(Debug, Clone)]
pub struct KeyChain {
  pubsub: APIKeyPubSub,
  db: Database,
  col: Collection<APIKey>,
}

impl KeyChain {
  pub async fn new(broker: &Client, db: Database) -> KeyChainResult<Self> {
    let col = db.collection("apiKeyChains");
    let ret = Self {
      pubsub: APIKeyPubSub::new(broker).await?,
      db,
      col,
    };
    ret.update_indices(&["exchange"]).await;
    return Ok(ret);
  }
}

#[async_trait]
impl IKeyChain for KeyChain {
  async fn push(&self, api_key: APIKey) -> KeyChainResult<Option<ObjectId>> {
    let result = self.col.insert_one(&api_key, None).await?;
    let id = result.inserted_id.as_object_id();
    let mut api_key = api_key.clone();
    api_key.inner_mut().id = id.clone();
    let event = APIKeyEvent::Add(api_key);
    let _ = self.pubsub.publish(&event).await?;
    return Ok(id.clone());
  }

  async fn rename_label(
    &self,
    id: ObjectId,
    label: &str,
  ) -> KeyChainResult<()> {
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

  async fn list(
    &self,
    filter: Document,
  ) -> KeyChainResult<BoxStream<'_, APIKey>> {
    let stream = self
      .col
      .find(filter, None)
      .await?
      .filter_map(|res| async { res.ok() })
      .boxed();
    return Ok(stream);
  }

  async fn get(
    &self,
    exchange: Exchanges,
    id: ObjectId,
  ) -> Result<Option<APIKey>> {
    let key = self
      .col
      .find_one(
        doc! {
          "_id": id,
          "exchange": exchange.as_str().to_lowercase(),
        },
        None,
      )
      .await?;
    return Ok(key);
  }

  async fn delete(&self, id: ObjectId) -> KeyChainResult<()> {
    if let Some(doc) =
      self.col.find_one_and_delete(doc! {"_id": id}, None).await?
    {
      let api_key: APIKey = doc;
      let event = APIKeyEvent::Remove(api_key);
      let _ = self.pubsub.publish(&event).await?;
    }
    return Ok(());
  }
}

impl DBWriterTrait for KeyChain {
  fn get_database(&self) -> &Database {
    return &self.db;
  }

  fn get_col_name(&self) -> &str {
    return self.col.name();
  }
}
