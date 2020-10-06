use ::mongodb::Database;
use ::nats::asynk::Connection as Broker;
use ::num_traits::FromPrimitive;
use ::slog::{o, Logger};
use ::tonic::{async_trait, Code, Request, Response, Status};

use ::exchanges::binance;
use ::exchanges::SymbolFetcher;
use ::rpc::entities::Exchanges;
use ::rpc::symbol::{symbol_server::Symbol, RefreshRequest};
use ::types::{rpc_ret_on_err, Result};

pub struct Service {
  binance: binance::SymbolFetcher,
}

impl Service {
  pub fn new(db: &Database, broker: Broker, log: Logger) -> Self {
    return Self {
      binance: binance::SymbolFetcher::new(
        log.new(o!("scope" => "BinanceSymbolFetcher")),
        broker.clone(),
        db.clone(),
      ),
    };
  }
}

#[async_trait]
impl Symbol for Service {
  async fn refresh(
    &self,
    request: Request<RefreshRequest>,
  ) -> Result<Response<()>> {
    let model = request.into_inner();
    let fetcher: &(dyn SymbolFetcher + Send + Sync) =
      match FromPrimitive::from_i32(model.exchange) {
        Some(Exchanges::Binance) => {
          &self.binance as &(dyn SymbolFetcher + Send + Sync)
        }
        _ => {
          return Err(Status::new(
            Code::NotFound,
            format!("No such symbol fetcher for the exchange"),
          ))
        }
      };
    rpc_ret_on_err!(Code::Internal, fetcher.refresh().await);
    return Ok(Response::new(()));
  }
}
