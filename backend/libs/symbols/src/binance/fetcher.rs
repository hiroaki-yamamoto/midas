use ::std::time::Duration as StdDur;

use ::async_trait::async_trait;
use ::futures::future::join;
use ::futures::stream::StreamExt;
use ::mongodb::bson::doc;
use ::mongodb::Database;
use ::url::Url;

use ::clients::binance::REST_ENDPOINTS;
use ::errors::{SymbolFetchError, SymbolFetchResult};
use ::round_robin_client::RestClient;
use ::rpc::symbol_info::SymbolInfo;
use ::subscribe::nats::Client as Nats;

use super::entities::{ExchangeInfo, Symbol};
use super::manager::SymbolUpdateEventManager;
use super::recorder::SymbolWriter;

use crate::traits::SymbolFetcher as SymbolFetcherTrait;
use ::errors::StatusFailure;

#[derive(Debug, Clone)]
pub struct SymbolFetcher {
  broker: Nats,
  recorder: SymbolWriter,
  cli: RestClient,
}

impl SymbolFetcher {
  pub async fn new(broker: Nats, db: &Database) -> SymbolFetchResult<Self> {
    let recorder = SymbolWriter::new(&db).await;
    let urls: SymbolFetchResult<Vec<Url>> = REST_ENDPOINTS
      .into_iter()
      .map(|&url| Ok((String::from(url) + "/api/v3/exchangeInfo").parse()?))
      .collect();
    let urls = urls?;
    let ret = Self {
      broker: broker,
      cli: RestClient::new(&urls, StdDur::from_secs(5), StdDur::from_secs(5))?,
      recorder,
    };
    return Ok(ret);
  }
}

#[async_trait]
impl SymbolFetcherTrait for SymbolFetcher {
  async fn refresh(&mut self) -> SymbolFetchResult<Vec<SymbolInfo>> {
    let resp = self.cli.get::<()>(None, None).await?;
    let old_symbols = self.recorder.list(doc! {}).await?;
    let old_symbols: Vec<Symbol> = old_symbols.collect().await;
    let resp_status = resp.status();
    if resp_status.is_success() {
      let info: ExchangeInfo = resp.json().await?;
      let new_symbols = info.symbols.clone();
      let update_event_manager = SymbolUpdateEventManager::new(
        &self.broker,
        new_symbols.clone(),
        old_symbols,
      )
      .await?;
      let (update, _) = join(
        self.recorder.update_symbols(new_symbols),
        update_event_manager.publish_changes(),
      )
      .await;
      update?;
      return Ok(
        self
          .recorder
          .list(doc! {})
          .await?
          .map(|item| item.into())
          .collect()
          .await,
      );
    } else {
      return Err(SymbolFetchError::HTTPErr(
        StatusFailure {
          url: Some(self.cli.get_current_url().to_string()),
          code: resp_status.as_u16(),
          text: resp.text().await?,
        }
        .into(),
      ));
    }
  }
}
