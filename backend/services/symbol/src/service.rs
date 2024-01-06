use ::futures::StreamExt;
use ::mongodb::Database;
use ::warp::filters::BoxedFilter;
use ::warp::reject;
use ::warp::{Filter, Rejection, Reply};

use ::errors::SymbolFetchResult;
use ::rpc::base_symbols::BaseSymbols;
use ::rpc::exchanges::Exchanges;
use ::rpc::status::Status;
use ::rpc::symbol_info::SymbolInfo;
use ::rpc::symbol_list::SymbolList;
use ::subscribe::nats::Client as Nats;
use ::symbols::binance::{
  fetcher as binance_fetcher, recorder as binance_recorder,
};
use ::symbols::traits::{SymbolFetcher, SymbolReader};

#[derive(Clone)]
pub struct Service {
  binance_fetcher: binance_fetcher::SymbolFetcher,
  binance_recorder: binance_recorder::SymbolWriter,
}

impl Service {
  pub async fn new(db: &Database, broker: Nats) -> SymbolFetchResult<Self> {
    return Ok(Self {
      binance_fetcher: binance_fetcher::SymbolFetcher::new(broker, db).await?,
      binance_recorder: binance_recorder::SymbolWriter::new(db).await,
    });
  }

  fn get_fetcher(
    &self,
    exchange: Exchanges,
  ) -> impl SymbolFetcher + Send + Sync {
    return match exchange {
      Exchanges::Binance => self.binance_fetcher.clone(),
    };
  }

  fn get_recorder(
    &self,
    exchange: Exchanges,
  ) -> impl SymbolReader + Send + Sync {
    return match exchange {
      Exchanges::Binance => self.binance_recorder.clone(),
    };
  }

  pub fn route(&self) -> BoxedFilter<(impl Reply,)> {
    return self
      .refresh()
      .or(self.base_currencies())
      .or(self.supported_currencies())
      .boxed();
  }

  fn base_currencies(
    &self,
  ) -> impl Filter<Extract = (impl Reply,), Error = ::warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static {
    let me = self.clone();
    return ::warp::path("base")
      .and(::warp::get())
      .and(Exchanges::by_param())
      .map(move |exchange| me.get_recorder(exchange))
      .and_then(handle_base_currencies)
      .map(|base: Vec<String>| {
        return Box::new(::warp::reply::json(&BaseSymbols { symbols: base }));
      });
  }

  fn supported_currencies(
    &self,
  ) -> impl Filter<Extract = (impl Reply,), Error = ::warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static {
    let me = self.clone();
    return ::warp::path("currencies")
      .and(::warp::get())
      .and(Exchanges::by_param())
      .map(move |exchange| me.get_recorder(exchange))
      .and_then(handle_supported_currencies)
      .map(|sym: Vec<SymbolInfo>| {
        let sym: Vec<Box<SymbolInfo>> =
          sym.into_iter().map(|sym| Box::new(sym.into())).collect();
        return Box::new(::warp::reply::json(&SymbolList { symbols: sym }));
      });
  }

  fn refresh(
    &self,
  ) -> impl Filter<Extract = (impl Reply,), Error = ::warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static {
    let me = self.clone();
    return ::warp::path("refresh")
      .and(::warp::post())
      .and(Exchanges::by_param())
      .map(move |exchange| me.get_fetcher(exchange))
      .and_then(handle_fetcher)
      .map(move |sym: Vec<SymbolInfo>| {
        let sym: Vec<Box<SymbolInfo>> =
          sym.into_iter().map(|sym| Box::new(sym.into())).collect();
        return Box::new(::warp::reply::json(&SymbolList { symbols: sym }));
      });
  }
}

async fn handle_supported_currencies(
  recorder: impl SymbolReader + Send + Sync,
) -> Result<Vec<SymbolInfo>, Rejection> {
  let symbols = recorder.list_all().await.map_err(|err| {
    reject::custom(Status::new(
      ::http::StatusCode::SERVICE_UNAVAILABLE,
      &format!("{}", err),
    ))
  })?;
  let symbols: Vec<SymbolInfo> = symbols.map(|sym| sym.into()).collect().await;
  return Ok(symbols);
}

async fn handle_base_currencies(
  recorder: impl SymbolReader + Send + Sync,
) -> Result<Vec<String>, Rejection> {
  return recorder.list_base_currencies().await.map_err(|err| {
    reject::custom(Status::new(
      ::http::StatusCode::SERVICE_UNAVAILABLE,
      &format!("{}", err),
    ))
  });
}

async fn handle_fetcher(
  mut fetcher: impl SymbolFetcher + Send + Sync,
) -> Result<Vec<SymbolInfo>, Rejection> {
  return fetcher
    .refresh()
    .await
    .map(|sym_list| sym_list.into_iter().map(|sym| sym.into()).collect())
    .map_err(|err| {
      reject::custom(Status::new(
        ::http::StatusCode::SERVICE_UNAVAILABLE,
        &format!("{}", err),
      ))
    });
}
