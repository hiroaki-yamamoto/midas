use ::futures::StreamExt;
use ::mongodb::Database;
use ::nats::Connection as Broker;
use ::slog::{o, Logger};
use ::warp::filters::BoxedFilter;
use ::warp::reject;
use ::warp::{Filter, Rejection, Reply};

use ::rpc::entities::{Exchanges, Status};
use ::rpc::symbols::SymbolList;
use ::symbols::binance::{
  fetcher as binance_fetcher, recorder as binance_recorder,
};
use ::symbols::traits::{Symbol as SymbolTrait, SymbolFetcher, SymbolRecorder};

#[derive(Clone)]
pub struct Service {
  binance_fetcher: binance_fetcher::SymbolFetcher,
  binance_recorder: binance_recorder::SymbolRecorder,
}

impl Service {
  pub async fn new(db: &Database, broker: Broker, log: Logger) -> Self {
    return Self {
      binance_fetcher: binance_fetcher::SymbolFetcher::new(
        log.new(o!("scope" => "BinanceSymbolFetcher")),
        broker.clone(),
        db.clone(),
      )
      .await,
      binance_recorder: binance_recorder::SymbolRecorder::new(db.clone()).await,
    };
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
  ) -> impl SymbolRecorder + Send + Sync {
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
        return Box::new(::warp::reply::json(&SymbolList { symbols: base }));
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
      .map(|symbols: Vec<String>| {
        return Box::new(::warp::reply::json(&SymbolList { symbols: symbols }));
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
      .map(move |sym| {
        return Box::new(::warp::reply::json(&SymbolList { symbols: sym }));
      });
  }
}

async fn handle_supported_currencies(
  recorder: impl SymbolRecorder + Send + Sync,
) -> Result<Vec<String>, Rejection> {
  let symbols = recorder.list(None).await.map_err(|err| {
    reject::custom(Status::new(
      ::warp::http::StatusCode::SERVICE_UNAVAILABLE,
      format!("{}", err),
    ))
  })?;
  let symbols = symbols.map(|sym| sym.symbol()).collect().await;
  return Ok(symbols);
}

async fn handle_base_currencies(
  recorder: impl SymbolRecorder + Send + Sync,
) -> Result<Vec<String>, Rejection> {
  return recorder.list_base_currencies().await.map_err(|err| {
    reject::custom(Status::new(
      ::warp::http::StatusCode::SERVICE_UNAVAILABLE,
      format!("{}", err),
    ))
  });
}

async fn handle_fetcher(
  fetcher: impl SymbolFetcher + Send + Sync,
) -> Result<Vec<String>, Rejection> {
  return fetcher.refresh().await.map_err(|err| {
    reject::custom(Status::new(
      ::warp::http::StatusCode::SERVICE_UNAVAILABLE,
      format!("{}", err),
    ))
  });
}
