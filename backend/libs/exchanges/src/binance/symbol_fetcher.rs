use ::async_trait::async_trait;
use ::futures::future::join;
use ::futures::stream::StreamExt;
use ::mongodb::bson::{
  de::Result as BsonDeResult, doc, from_bson, to_bson, Array, Bson, Document,
};
use ::mongodb::{error::Result as MongoResult, Collection, Database};
use ::nats::asynk::Connection as Broker;
use ::rmp_serde::to_vec as to_msgpack;
use ::rpc::entities::SymbolInfo;
use ::slog::Logger;
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::{REST_ENDPOINT, SYMBOL_UPDATE_EVENT};
use super::entities::{ExchangeInfo, Symbol, SymbolUpdateEvent};
use crate::entities::ListSymbolStream;
use crate::errors::StatusFailure;
use crate::traits::SymbolFetcher as SymbolFetcherTrait;

const SYMBOL_FETCHER_RECORD_COL_NAME: &'static str = "binance.symbol";

#[derive(Debug, Clone)]
pub struct SymbolFetcher {
  broker: Broker,
  col: Collection,
  log: Logger,
}

impl SymbolFetcher {
  pub fn new(log: Logger, broker: Broker, db: Database) -> Self {
    return Self {
      broker,
      col: db.collection(SYMBOL_FETCHER_RECORD_COL_NAME),
      log,
    };
  }
  pub async fn get(
    &self,
    filter: impl Into<Option<Document>> + Send,
  ) -> SendableErrorResult<Vec<SymbolInfo>> {
    let cur = ret_on_err!(self.col.find(filter, None).await);
    let mut docs: Vec<MongoResult<Document>> = cur.collect().await;
    docs.retain(|doc| doc.is_ok());
    let symbols: Vec<BsonDeResult<Symbol>> = docs
      .iter()
      .map(|doc_res| {
        let doc = doc_res.clone().unwrap();
        let item: BsonDeResult<Symbol> = from_bson(Bson::Document(doc));
        return item;
      })
      .filter(|item| item.is_ok())
      .collect();
    let ret = symbols
      .into_iter()
      .map(|item| item.unwrap().as_symbol_info())
      .collect();
    return Ok(ret);
  }

  async fn publish_update_event<S, T>(&self, new_symbols: S, old_symbols: T)
  where
    S: IntoIterator<Item = String>,
    T: IntoIterator<Item = String>,
  {
    let diff =
      SymbolUpdateEvent::new(new_symbols.into_iter(), old_symbols.into_iter());
    let msg = match to_msgpack(&diff) {
      Err(e) => {
        ::slog::error!(
          self.log,
          "Failed to encode the payload for update event: {}",
          e
        );
        return;
      }
      Ok(v) => v,
    };
    match self.broker.publish(SYMBOL_UPDATE_EVENT, &msg[..]).await {
      Err(e) => {
        ::slog::error!(self.log, "Failed to publish the update event: {}", e);
        return;
      }
      Ok(_) => {}
    }
  }
}

#[async_trait]
impl SymbolFetcherTrait for SymbolFetcher {
  async fn refresh(&self) -> SendableErrorResult<()> {
    let mut url: url::Url = ret_on_err!(REST_ENDPOINT.parse());
    url = ret_on_err!(url.join("/api/v3/exchangeInfo"));
    let resp = ret_on_err!(reqwest::get(url.clone()).await);
    let old_symbols: Vec<String> =
      ret_on_err!(self.col.find(doc! {}, None).await)
        .filter_map(|res| async { res.ok() })
        .filter_map(|doc| async move { doc.get("symbol").cloned() })
        .filter_map(|sym| async move { sym.as_str().map(|s| String::from(s)) })
        .collect()
        .await;
    let resp_status = resp.status();
    if resp_status.is_success() {
      let info: ExchangeInfo = ret_on_err!(resp.json().await);
      let new_symbols =
        info.symbols.clone().into_iter().map(|info| info.symbol);
      let update_event = self.publish_update_event(new_symbols, old_symbols);
      ret_on_err!(self.col.delete_many(doc! {}, None).await);
      let empty = Array::new();
      let serialized: Vec<Document> = ret_on_err!(to_bson(&info.symbols))
        .as_array()
        .unwrap_or(&empty)
        .into_iter()
        .filter_map(|item| item.as_document())
        .map(|item| item.clone())
        .collect();
      let (ins_res, _) = join(
        self.col.insert_many(serialized.into_iter(), None),
        update_event,
      )
      .await;
      ret_on_err!(ins_res);
      return Ok(());
    } else {
      return Err(Box::new(StatusFailure {
        url: url.clone(),
        code: resp_status.as_u16(),
        text: ret_on_err!(resp.text().await),
      }));
    }
  }

  type ListStream = ListSymbolStream;
  async fn list(&self) -> SendableErrorResult<Self::ListStream> {
    let cur = ret_on_err!(self.col.find(doc! {}, None).await);
    let cur = cur
      .filter_map(|doc| async { doc.ok() })
      .map(|doc| from_bson::<Symbol>(Bson::Document(doc)))
      .filter_map(|doc| async { doc.ok() })
      .map(|item| item.as_symbol_info())
      .boxed();
    return Ok(Box::pin(cur) as Self::ListStream);
  }
}
