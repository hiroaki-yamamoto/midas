use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::futures::stream::{BoxStream, Stream};
use ::mongodb::bson::{doc, oid::ObjectId, Document};
use ::mongodb::results::InsertManyResult;
use ::mongodb::Database;
use ::nats::asynk::{Connection as NatsCon, Subscription};
use ::serde::Serialize;

use ::types::{GenericResult, SendableErrorResult};

use super::entities::{BookTicker, ExecutionResult, OrderOption};

#[async_trait]
pub trait Recorder {
  fn get_database(&self) -> &Database;
  fn get_col_name(&self) -> &str;
  async fn update_indices(&self, flds: &[&str]) {
    let col_name = self.get_col_name();
    let db = self.get_database();
    let has_index = db
      .run_command(doc! {"listIndexes": &col_name}, None)
      .await
      .map(|item| {
        return item
          .get_document("listIndexes")
          .unwrap_or(&doc! {"ok": false})
          .get_bool("ok")
          .unwrap_or(false);
      })
      .unwrap_or(false);
    if !has_index {
      let mut indices = vec![];
      for fld_name in flds {
        indices.push(doc! { "name": format!("{}_index", fld_name), "key": doc!{
          *fld_name: 1,
        } })
      }
      let _ = db
        .run_command(
          doc! {
            "createIndexes": &col_name,
            "indexes": indices
          },
          None,
        )
        .await;
    }
  }
}

#[async_trait]
pub trait HistoryFetcher {
  async fn refresh(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<Subscription>;
  async fn stop(&self) -> SendableErrorResult<()>;
  async fn spawn(&self) -> SendableErrorResult<()>;
}

#[async_trait]
pub trait SymbolFetcher {
  async fn refresh(&self) -> SendableErrorResult<()>;
}

#[async_trait]
pub trait HistoryRecorder {
  async fn spawn(&self);
}

#[async_trait]
pub trait SymbolRecorder {
  type ListStream: Stream + Send + 'static;
  async fn list(
    &self,
    query: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> SendableErrorResult<Self::ListStream>;
  async fn update_symbols<T>(
    &self,
    value: Vec<T>,
  ) -> SendableErrorResult<InsertManyResult>
  where
    T: Serialize + Send;
}

#[async_trait]
pub trait TradeObserver {
  async fn start(&self) -> SendableErrorResult<()>;
  async fn subscribe(&self) -> ::std::io::Result<BoxStream<'_, BookTicker>>;
}

pub(crate) trait TradeDateTime {
  fn symbol(&self) -> String;
  fn open_time(&self) -> DateTime<Utc>;
  fn close_time(&self) -> DateTime<Utc>;
}

#[async_trait]
pub trait Executor {
  async fn create_order(
    &mut self,
    symbol: String,
    price: Option<f64>,
    budget: f64,
    order_option: Option<OrderOption>,
  ) -> GenericResult<ObjectId>;

  async fn remove_order(
    &mut self,
    id: ObjectId,
  ) -> GenericResult<ExecutionResult>;
}
