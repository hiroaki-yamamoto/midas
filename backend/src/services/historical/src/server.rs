use ::std::pin::Pin;

use ::futures::Stream;

use ::mongodb::Database;
use ::nats::Connection as NatsCon;
use ::rmp_serde::{from_slice as read_msgpack, to_vec as serialize_msgpack};
use ::slog::{error, o, Logger};
use ::tonic::{async_trait, Code, Request, Response, Status};

use ::exchanges::{Binance, KlineCtrl};
use ::rpc::historical::{
  hist_chart_server::HistChart, HistChartFetchReq, HistChartProg, StopRequest,
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
  pub fn new(log: &Logger, db: &Database, nats: NatsCon) -> Self {
    let log = log.new(o!("scope" => "History Fetch RPC Server"));
    return Self {
      logger: log.clone(),
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
    _: tonic::Request<()>,
  ) -> Result<tonic::Response<Self::subscribeStream>> {
    let manager = ExchangeManager::new(
      String::from("binance"),
      &self.binance,
      &self.nats,
      self.logger.new(o!("scope" => "Binance Exchange Manager")),
    );
    let subscriber = rpc_ret_on_err!(Code::Internal, manager.subscribe());
    let stream_logger = self.logger.new(o!("scope" => "Stream Logger"));
    let out = ::async_stream::try_stream! {
      while let Some(msg) = subscriber.next() {
        let prog: HistChartProg = match read_msgpack(&msg.data[..]) {
          Err(e) => {
            error!(
              stream_logger,
              "Got an error while deserializing HistFetch Prog. {}",
              e
            );
            continue;
          },
          Ok(v) => v,
        };
        yield prog;
      }
    };
    return Ok(Response::new(Box::pin(out) as Self::subscribeStream));
  }

  async fn stop(
    &self,
    request: tonic::Request<StopRequest>,
  ) -> Result<tonic::Response<()>> {
    let req = request.into_inner();
    let msg =
      rpc_ret_on_err!(Code::Internal, serialize_msgpack(&KlineCtrl::Stop));
    for exc in req.exchanges {
      let _ = rpc_ret_on_err!(
        Code::Internal,
        self
          .nats
          .publish(&format!("{}.kline.ctrl", exc.to_string()), &msg)
      );
    }
    return Ok(Response::new(()));
  }
}
