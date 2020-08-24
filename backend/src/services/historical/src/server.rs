use ::std::pin::Pin;

use ::futures::Stream;

use ::mongodb::Database;
use ::nats::Connection as NatsCon;
use ::slog::{o, warn, Logger};
use ::tonic::{async_trait, Code, Request, Response, Status};

use ::exchanges::Binance;
use ::rpc::historical::{
  hist_chart_server::HistChart, HistChartFetchReq, HistChartProg,
};
use ::types::{rpc_ret_on_err, Result};

use super::manager::ExchangeManager;

#[derive(Debug)]
pub struct Server {
  logger: Logger,
  binance: Binance,
  nats: NatsCon,
}

impl Server {
  fn new(log: Logger, db: &Database, nats: NatsCon) -> Self {
    return Self {
      logger: log,
      binance: Binance::new(
        log.new(o!("Exchange" => "Binance")),
        db.collection("binance.history"),
        db.collection("binance.symbolinfo"),
      ),
      nats,
    };
  }
}

#[async_trait]
impl HistChart for Server {
  async fn sync(
    &self,
    req: Request<HistChartFetchReq>,
  ) -> Result<Response<()>> {
    let req = req.into_inner();
    let manager = ExchangeManager::new(
      String::from("binance"),
      &self.binance,
      &self.nats,
      self.logger.new(o!("scope" => "Binance Exchange Manager")),
    );
    rpc_ret_on_err!(
      Code::Internal,
      manager.refresh_historical_klines(req.symbols).await
    );
    return Ok(Response::new(()));
  }

  type subscribeStream =
    Pin<Box<dyn Stream<Item = Result<HistChartProg>> + Send + Sync + 'static>>;
  async fn subscribe(
    &self,
    request: tonic::Request<()>,
  ) -> Result<tonic::Response<Self::subscribeStream>> {
    let manager = ExchangeManager::new(
      String::from("binance"),
      &self.binance,
      &self.nats,
      self.logger.new(o!("scope" => "Binance Exchange Manager")),
    );
    let subscriber = manager.subscribe();
  }
}
