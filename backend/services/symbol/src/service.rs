use ::std::pin::Pin;

use ::mongodb::Database;
use ::nats::asynk::Connection as Broker;
use ::slog::{o, Logger};
use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Rejection, Reply};
use warp::reject::Reject;

use ::exchanges::binance;
use ::exchanges::SymbolFetcher;
use ::num_traits::FromPrimitive;
use ::rpc::entities::Exchanges;

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
  ) -> Result<impl SymbolFetcher + Send + Sync, Rejection> {
    return match exchange {
      Exchanges::Binance => Ok(self.binance.clone()),
      Exchanges::Unknown => Err(::warp::reject::not_found()),
    };
  }

  pub async fn route(&self) -> BoxedFilter<(impl Reply,)> {
    return self.refresh().await.boxed();
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
      .and(::warp::path::param::<u16>())
      .and_then(|param: u16| async move {
        let exchange: Exchanges = match FromPrimitive::from_u16(param) {
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

async fn handle_fetcher(
  fetcher: Result<impl SymbolFetcher + Send + Sync, Rejection>,
) -> Result<(), Rejection> {
  let fetcher = fetcher?;
  let _ = fetcher.refresh().await;
  return Ok(());
}
