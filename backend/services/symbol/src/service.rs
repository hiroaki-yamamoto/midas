use ::futures::StreamExt;
use ::mongodb::Database;
use ::nats::asynk::Connection as Broker;
use ::num_traits::FromPrimitive;
use ::slog::{o, Logger};
use ::tonic::{async_trait, Code, Request, Response, Status};

use ::exchanges::binance;
use ::exchanges::{ListSymbolStream, SymbolFetcher};
use ::rpc::entities::Exchanges;
use ::rpc::symbol::{
  symbol_server::Symbol, QueryRequest, QueryResponse, RefreshRequest,
};
use ::types::{rpc_ret_on_err, Result};

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
    exchange: Option<Exchanges>,
  ) -> Result<&(dyn SymbolFetcher<ListStream = ListSymbolStream> + Send + Sync)>
  {
    let fetcher: &(dyn SymbolFetcher<ListStream = ListSymbolStream>
        + Send
        + Sync) = match exchange {
      Some(Exchanges::Binance) => {
        &self.binance
          as &(dyn SymbolFetcher<ListStream = ListSymbolStream> + Send + Sync)
      }
      _ => {
        return Err(Status::new(
          Code::NotFound,
          format!("No such symbol fetcher for the exchange"),
        ))
      }
    };
    return Ok(fetcher);
  }
}

#[async_trait]
impl Symbol for Service {
  async fn refresh(
    &self,
    request: Request<RefreshRequest>,
  ) -> Result<Response<()>> {
    let model = request.into_inner();
    let fetcher = self.get_fetcher(FromPrimitive::from_i32(model.exchange))?;
    rpc_ret_on_err!(Code::Internal, fetcher.refresh().await);
    return Ok(Response::new(()));
  }

  async fn query(
    &self,
    request: tonic::Request<QueryRequest>,
  ) -> Result<tonic::Response<QueryResponse>> {
    let request = request.into_inner();
    let fetcher =
      self.get_fetcher(FromPrimitive::from_i32(request.exchange))?;
    let status = match request.status.trim() {
      "" => None,
      o => Some(String::from(o)),
    };
    let symbols: Option<Vec<String>>;
    if request.symbols.len() > 0 {
      symbols = Some(request.symbols);
    } else {
      symbols = None;
    }
    let list_st =
      rpc_ret_on_err!(Code::Internal, fetcher.list(status, symbols).await);
    let ret = list_st.collect().await;
    return Ok(Response::new(QueryResponse { symbols: ret }));
  }
}
