use ::async_trait::async_trait;
use ::futures::future::join;
use ::futures::stream::StreamExt;
use ::mongodb::bson::{doc, Document};
use ::mongodb::Database;
use ::nats::asynk::Connection as Broker;
use ::slog::Logger;

use ::rpc::entities::SymbolInfo;
use ::types::ThreadSafeResult;

use super::super::constants::REST_ENDPOINT;
use super::super::entities::{ExchangeInfo, Symbol};
use super::super::managers::SymbolUpdateEventManager;
use super::super::recorders::SymbolRecorder;

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
  ) -> ThreadSafeResult<Vec<SymbolInfo>> {
    let docs = self.recorder.list(filter).await?;
    let docs: Vec<SymbolInfo> = docs.map(|doc| doc.into()).collect().await;
    return Ok(docs);
  }
}

#[async_trait]
impl SymbolFetcherTrait for SymbolFetcher {
  async fn refresh(&self) -> ThreadSafeResult<()> {
    let mut url: url::Url = REST_ENDPOINT.parse()?;
    url = url.join("/api/v3/exchangeInfo")?;
    let resp = reqwest::get(url.clone()).await?;
    let old_symbols = self.recorder.list(doc! {}).await?;
    let old_symbols: Vec<Symbol> = old_symbols.collect().await;
    let resp_status = resp.status();
    if resp_status.is_success() {
      let info: ExchangeInfo = resp.json().await?;
      let new_symbols = info.symbols.clone();
      let update_event_manager = SymbolUpdateEventManager::new(
        &self.log,
        &self.broker,
        new_symbols.clone(),
        old_symbols,
      );
      let (update, _) = join(
        self.recorder.update_symbols(new_symbols),
        update_event_manager.publish_changes(),
      )
      .await;
      update?;
      return Ok(());
    } else {
      return Err(Box::new(StatusFailure {
        url: url.clone(),
        code: resp_status.as_u16(),
        text: resp.text().await?,
      }));
    }
  }
}
