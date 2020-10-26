use ::async_trait::async_trait;
use ::chrono::{DateTime, Utc};
use ::futures::stream::Stream;
use ::mongodb::bson::doc;
use ::mongodb::Database;
use ::nats::asynk::Subscription;

use ::rpc::entities::SymbolInfo;
use ::types::SendableErrorResult;

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
  type ListStream: Stream<Item = SymbolInfo> + Send + 'static;
  async fn refresh(&self) -> SendableErrorResult<()>;
  async fn list(
    &self,
    status: Option<String>,
    symbols: Option<Vec<String>>,
  ) -> SendableErrorResult<Self::ListStream>;
}

#[async_trait]
pub trait HistoryRecorder {
  async fn spawn(&self);
}

#[async_trait]
pub trait TradeObserver {
  async fn start(&self) -> SendableErrorResult<()>;
  async fn subscribe(&self) -> ::std::io::Result<Subscription>;
}

pub(crate) trait TradeDateTime {
  fn symbol(&self) -> String;
  fn open_time(&self) -> DateTime<Utc>;
  fn close_time(&self) -> DateTime<Utc>;
}
