use ::futures::executor::block_on;
use ::mongodb::Database;
use ::nats::asynk::Connection as Broker;
use ::slog::{o, Logger};
use ::warp::filters::BoxedFilter;
use ::warp::http::StatusCode;
use ::warp::Filter;
use ::warp::Reply;

use ::exchanges::binance;
use ::exchanges::{ListSymbolStream, SymbolFetcher};
use ::rpc::entities::Exchanges;
use ::types::reply_on_err;

#[derive(Clone)]
pub struct Service {
  binance: binance::SymbolFetcher,
}

impl Service {
  pub async fn new(db: &Database, broker: Broker, log: Logger) -> Self {
    return Self {
      binance: binance::SymbolFetcher::new(
        log.new(o!("scope" => "BinanceSymbolFetcher")),
        broker.clone(),
        db.clone(),
      )
      .await,
    };
  }

  fn get_fetcher(
    &self,
    exchange: Exchanges,
  ) -> &(dyn SymbolFetcher<ListStream = ListSymbolStream<'static>> + Send + Sync)
  {
    let fetcher: &(dyn SymbolFetcher<ListStream = ListSymbolStream<'static>>
        + Send
        + Sync) = match exchange {
      Exchanges::Binance => {
        &self.binance
          as &(dyn SymbolFetcher<ListStream = ListSymbolStream> + Send + Sync)
      }
    };
    return fetcher;
  }

  pub fn route(&self) -> BoxedFilter<(impl Reply,)> {
    return self.refresh().boxed();
  }

  fn refresh(&self) -> BoxedFilter<(impl Reply,)> {
    let me = self.clone();
    return ::warp::path("refresh")
      .and(::warp::path::param())
      .map(move |exchange: Exchanges| {
        let fetcher = me.get_fetcher(exchange);
        let _ = reply_on_err!(
          StatusCode::INTERNAL_SERVER_ERROR,
          block_on(fetcher.refresh())
        );
        return Box::new(::warp::reply());
      })
      .boxed();
  }
}
