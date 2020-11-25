use ::async_trait::async_trait;
use ::futures::future::join;
use ::futures::stream::StreamExt;
use ::mongodb::bson::{doc, Document};
use ::mongodb::Database;
use ::nats::asynk::Connection as Broker;
use ::slog::Logger;

use ::rpc::entities::SymbolInfo;
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::REST_ENDPOINT;
use super::entities::{ExchangeInfo, Symbol};
use super::managers::SymbolUpdateEventManager;
use super::symbol_recorder::SymbolRecorder;

use crate::errors::StatusFailure;
use crate::traits::{
  SymbolFetcher as SymbolFetcherTrait, SymbolRecorder as SymbolRecorderTrait,
};

#[derive(Debug, Clone)]
pub struct SymbolFetcher {
  broker: Broker,
  recorder: SymbolRecorder,
  log: Logger,
}

impl SymbolFetcher {
  pub async fn new(log: Logger, broker: Broker, db: Database) -> Self {
    let recorder = SymbolRecorder::new(db).await;
    let ret = Self {
      broker,
      recorder,
      log,
    };
    return ret;
  }

  pub async fn get(
    &self,
    filter: impl Into<Option<Document>> + Send,
  ) -> SendableErrorResult<Vec<SymbolInfo>> {
    let docs: Vec<SymbolInfo> = self
      .recorder
      .list(filter)
      .await?
      .map(|doc| doc.into())
      .collect()
      .await;
    return Ok(docs);
  }
}

#[async_trait]
impl SymbolFetcherTrait for SymbolFetcher {
  async fn refresh(&self) -> SendableErrorResult<()> {
    let mut url: url::Url = ret_on_err!(REST_ENDPOINT.parse());
    url = ret_on_err!(url.join("/api/v3/exchangeInfo"));
    let resp = ret_on_err!(reqwest::get(url.clone()).await);
    let old_symbols: Vec<Symbol> =
      self.recorder.list(doc! {}).await?.collect().await;
    let resp_status = resp.status();
    if resp_status.is_success() {
      let info: ExchangeInfo = ret_on_err!(resp.json().await);
      let new_symbols = info.symbols.clone();
      let update_event_manager = SymbolUpdateEventManager::new(
        &self.log,
        &self.broker,
        new_symbols.clone(),
        old_symbols,
      );
      let update_event = update_event_manager.publish_changes();
      let update = self.recorder.update_symbols(new_symbols);
      let (ins_res, _) = join(update, update_event).await;
      ins_res?;
      return Ok(());
    } else {
      return Err(Box::new(StatusFailure {
        url: url.clone(),
        code: resp_status.as_u16(),
        text: ret_on_err!(resp.text().await),
      }));
    }
  }
}
