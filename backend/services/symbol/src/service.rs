use ::std::convert::TryFrom;

use ::mongodb::Database;
use ::nats::Connection as Broker;
use ::slog::{o, Logger};
use ::warp::filters::BoxedFilter;
use ::warp::reject;
use ::warp::{Filter, Rejection, Reply};

use ::binance_symbols::{
  fetcher as binance_fetcher, recorder as binance_recorder, SymbolFetcher,
  SymbolRecorder,
};
use ::errors::StatusFailure;
use ::num_traits::FromPrimitive;
use ::rpc::entities::Exchanges;

use super::entities::BaseCurrencies;

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

  pub async fn route(&self) -> BoxedFilter<(impl Reply,)> {
    return self.refresh().await.or(self.base_currencies()).boxed();
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
      .and(::warp::path::param::<i32>())
      .and_then(|param: i32| async move {
        return Exchanges::try_from(param)
          .map(|exchange| (exchange,))
          .map_err(|_| ::warp::reject::not_found());
      })
      .untuple_one()
      .map(move |exchange: Exchanges| me.get_recorder(exchange))
      .and_then(handle_base_currencies)
      .map(|base: Vec<String>| {
        return Box::new(::warp::reply::json(&BaseCurrencies::new(base)));
      });
  }

  async fn refresh(
    &self,
  ) -> impl Filter<Extract = (impl Reply,), Error = ::warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static {
    let me = self.clone();
    return ::warp::path("refresh")
      .and(::warp::post())
      .and(::warp::path::param::<i32>())
      .and_then(|param: i32| async move {
        let exchange: Exchanges = match FromPrimitive::from_i32(param) {
          Some(v) => v,
          None => {
            return Err(::warp::reject::not_found());
          }
        };
        return Ok((exchange,));
      })
      .untuple_one()
      .map(move |exchange: Exchanges| me.get_fetcher(exchange))
      .and_then(handle_fetcher)
      .map(move |_| {
        return Box::new(::warp::reply());
      });
  }
}

async fn handle_base_currencies(
  recorder: impl SymbolRecorder + Send + Sync,
) -> Result<Vec<String>, Rejection> {
  return recorder.list_base_currencies().await.map_err(|err| {
    reject::custom(StatusFailure::new(None, 500, format!("{}", err)))
  });
}

async fn handle_fetcher(
  fetcher: impl SymbolFetcher + Send + Sync,
) -> Result<(), Rejection> {
  let _ = fetcher.refresh().await;
  return Ok(());
}
