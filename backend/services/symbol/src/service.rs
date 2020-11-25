use ::std::pin::Pin;

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
use ::num_traits::FromPrimitive;
use ::rpc::entities::Exchanges;
use ::types::reply_on_err;

#[derive(Clone)]
pub struct Service {
  binance: binance::SymbolFetcher,
}

type Fetcher =
  Box<dyn SymbolFetcher<ListStream = ListSymbolStream<'static>> + Send + Sync>;

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

  fn get_fetcher(&self, exchange: Exchanges) -> Fetcher {
    let fetcher: Fetcher = match exchange {
      Exchanges::Binance => Box::new(self.binance.clone()),
    };
    return fetcher;
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
      .map(move |exchange: Exchanges| Box::pin(me.get_fetcher(exchange)))
      .and_then(handle_fetcher)
      .map(move |_| {
        return Box::new(::warp::reply());
      });
  }
}

async fn handle_fetcher(
  fetcher: Pin<Box<Fetcher>>,
) -> Result<(), ::std::convert::Infallible> {
  let _ = fetcher.refresh().await;
  return Ok(());
}
