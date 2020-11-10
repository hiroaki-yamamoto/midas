use ::async_trait::async_trait;
use ::futures::future::{join, join_all};
use ::futures::stream::StreamExt;
use ::mongodb::bson::{
  de::Result as BsonDeResult, doc, from_bson, to_bson, Array, Bson, Document,
};
use ::mongodb::{error::Result as MongoResult, Collection, Database};
use ::nats::asynk::Connection as Broker;
use ::rmp_serde::to_vec as to_msgpack;
use ::slog::Logger;

use ::rpc::entities::SymbolInfo;
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::{REST_ENDPOINT, SYMBOL_INIT_EVENT};
use super::entities::{ExchangeInfo, Symbol};
use super::managers::SymbolUpdateEventManager;

use crate::entities::ListSymbolStream;
use crate::errors::StatusFailure;
use crate::traits::{Recorder, SymbolFetcher as SymbolFetcherTrait};

const SYMBOL_FETCHER_RECORD_COL_NAME: &'static str = "binance.symbol";

#[derive(Debug, Clone)]
pub struct SymbolFetcher {
  broker: Broker,
  col: Collection,
  db: Database,
  log: Logger,
}

impl Recorder for SymbolFetcher {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return &self.col.name();
  }
}

impl SymbolFetcher {
  pub async fn new(log: Logger, broker: Broker, db: Database) -> Self {
    let col = db.collection(SYMBOL_FETCHER_RECORD_COL_NAME);
    let ret = Self {
      broker,
      db,
      col,
      log,
    };
    ret.update_indices(&["symbol"]).await;
    ret.publish_init_event().await;
    return ret;
  }

  pub async fn publish_init_event(&self) {
    let mut cur = match self.col.find(doc! {}, None).await {
      Err(e) => {
        ::slog::warn!(self.log, "Failed to read symbol data from db: {}", e);
        return;
      }
      Ok(c) => c,
    }
    .filter_map(|doc| async { doc.ok() })
    .map(|doc| from_bson::<Symbol>(Bson::Document(doc)))
    .filter_map(|doc| async { doc.ok() })
    .map(|doc| to_msgpack(&doc))
    .filter_map(|data| async { data.ok() })
    .boxed();
    let mut pub_defer = vec![];
    while let Some(data) = cur.next().await {
      pub_defer.push(self.broker.publish(SYMBOL_INIT_EVENT, data));
    }
    join_all(pub_defer).await;
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
}

#[async_trait]
impl SymbolFetcherTrait for SymbolFetcher {
  async fn refresh(&self) -> SendableErrorResult<()> {
    let mut url: url::Url = ret_on_err!(REST_ENDPOINT.parse());
    url = ret_on_err!(url.join("/api/v3/exchangeInfo"));
    let resp = ret_on_err!(reqwest::get(url.clone()).await);
    let old_symbols: Vec<Symbol> =
      ret_on_err!(self.col.find(doc! {}, None).await)
        .filter_map(|res| async { res.ok() })
        .filter_map(|doc| async move {
          from_bson::<Symbol>(Bson::Document(doc)).ok()
        })
        .collect()
        .await;
    let resp_status = resp.status();
    if resp_status.is_success() {
      let info: ExchangeInfo = ret_on_err!(resp.json().await);
      let new_symbols = info.symbols.clone();
      let update_event_manager = SymbolUpdateEventManager::new(
        &self.log,
        &self.broker,
        new_symbols,
        old_symbols,
      );
      let update_event = update_event_manager.publish_changes();
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

  type ListStream = ListSymbolStream<'static>;
  async fn list(
    &self,
    status: Option<String>,
    symbols: Option<Vec<String>>,
  ) -> SendableErrorResult<Self::ListStream> {
    let mut query = doc! {};
    if let Some(status) = status {
      query.insert("status", status);
    }
    if let Some(symbols) = symbols {
      query.insert("symbol", doc! {"$in": symbols});
    }
    let cur = ret_on_err!(self.col.find(query, None).await);
    let cur = cur
      .filter_map(|doc| async { doc.ok() })
      .map(|doc| from_bson::<Symbol>(Bson::Document(doc)))
      .filter_map(|doc| async { doc.ok() })
      .map(|item| item.as_symbol_info())
      .boxed();
    return Ok(Box::pin(cur) as Self::ListStream);
  }
}
