use ::async_trait::async_trait;
use ::futures::stream::StreamExt;
use ::mongodb::bson::{
  de::Result as BsonDeResult, doc, from_bson, to_bson, Array, Bson, Document,
};
use ::mongodb::{error::Result as MongoResult, Collection};
use ::rpc::entities::SymbolInfo;
use ::slog::Logger;
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::REST_ENDPOINT;
use super::entities::{ExchangeInfo, Symbol};
use crate::errors::StatusFailure;
use crate::traits::SymbolFetcher as SymbolFetcherTrait;

#[derive(Debug, Clone)]
pub struct SymbolFetcher {
  col: Collection,
  log: Logger,
}

impl SymbolFetcher {
  pub fn new(log: Logger, col: Collection) -> Self {
    return Self { col, log };
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
    let resp_status = resp.status();
    if resp_status.is_success() {
      let info: ExchangeInfo = ret_on_err!(resp.json().await);
      ret_on_err!(self.col.delete_many(doc! {}, None).await);
      let empty = Array::new();
      let serialized: Vec<Document> = ret_on_err!(to_bson(&info.symbols))
        .as_array()
        .unwrap_or(&empty)
        .into_iter()
        .filter_map(|item| item.as_document())
        .map(|item| item.clone())
        .collect();
      ret_on_err!(self.col.insert_many(serialized.into_iter(), None).await);
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
