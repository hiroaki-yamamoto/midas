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

impl<'nats> Server {
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
  type syncStream =
    Pin<Box<dyn Stream<Item = Result<HistChartProg>> + Send + Sync + 'static>>;

  async fn sync(
    &self,
    req: Request<HistChartFetchReq>,
  ) -> Result<Response<Self::syncStream>> {
    let req = req.into_inner();
    let mut manager = ExchangeManager::new(
      String::from("binance"),
      self.binance.clone(),
      self.nats.clone(),
      self.logger.new(o!("scope" => "Binance Exchange Manager")),
    );
    let (stop, mut progress) = rpc_ret_on_err!(
      Code::Internal,
      manager.refresh_historical_klines(req.symbols).await
    );
    let recv_log = self.logger.new(o!("scope" => "recv"));
    let out = async_stream::try_stream! {
      while !manager.is_completed() {
        let data = match progress.recv().await {
          None => continue,
          Some(d) => match d {
            Err(e) => {
              warn!(recv_log, "Got an error: {}", e);
              continue;
            },
            Ok(v) => v
          },
        };
        yield data.clone();
      }
      stop.send(());
    };
    return Ok(Response::new(Box::pin(out) as Self::syncStream));
  }
}
