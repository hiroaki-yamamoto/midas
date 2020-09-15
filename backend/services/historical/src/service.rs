use ::std::pin::Pin;
use ::std::result::Result as StdResult;

use ::futures::future::join_all;
use ::futures::{SinkExt, Stream, StreamExt};
use ::mongodb::Database;
use ::nats::Connection as NatsCon;
use ::num_traits::FromPrimitive;
use ::rmp_serde::from_slice as read_msgpack;
use ::serde_json::to_string as jsonify;
use ::slog::{error, o, Logger};
use ::tonic::{async_trait, Code, Request, Response};
use ::warp::ws::{Message, WebSocket, Ws};

use ::exchanges::{binance, HistoryFetcher};
use ::rpc::entities::Exchanges;
use ::rpc::historical::{
  hist_chart_server::HistChart, HistChartFetchReq, HistChartProg, StopRequest,
};
use ::types::{rpc_ret_on_err, GenericResult, Result, Status};
use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Reply};

use super::manager::ExchangeManager;

use super::entities::KlineFetchStatus;

#[derive(Debug, Clone)]
pub struct Service {
  logger: Logger,
  binance: binance::HistoryFetcher,
  nats: NatsCon,
}

impl Service {
  pub fn new(
    log: &Logger,
    db: &Database,
    nats: NatsCon,
  ) -> GenericResult<Self> {
    let log = log.new(o!("scope" => "History Fetch RPC Service"));
    return Ok(Self {
      logger: log.clone(),
      binance: binance::HistoryFetcher::new(
        None,
        db.collection("binance.history"),
        log.new(o!("exchange" => "Binance", "scope" => "HistoryFetch")),
        nats.clone(),
        binance::SymbolFetcher::new(
          log.new(o!("exchange" => "Binance", "scope" => "SymbolFetch")),
          db.collection("binance.symbol"),
        ),
      )?,
      nats,
    });
  }

  pub fn get_websocket_route(&self) -> BoxedFilter<(impl Reply,)> {
    let ws_svc = self.clone();
    return ::warp::path("subscribe")
      .and(::warp::ws())
      .map(move |ws: Ws| {
        let ws_svc = ws_svc.clone();
        return ws.on_upgrade(|mut sock: WebSocket| async move {
          let subsc = ws_svc.subscribe(Request::new(())).await;
          match subsc {
            Err(e) => {
              let _ = sock.send(Message::close_with(
                1011 as u16,
                format!(
                  "Got an error while trying to subscribe the channel: {}",
                  e
                ),
              ));
              let _ = sock.flush();
            }
            Ok(resp) => {
              // let mut stream: Pin<Box<dyn Stream<Item = Message>>> = Box::pin(
              //   resp
              //     .into_inner()
              //     .map(|r| {
              //       return r
              //         .map(|d| {
              //           return jsonify(&d).unwrap_or(String::from(
              //             "Failed to serialize the progress data.",
              //           ));
              //         })
              //         .map_err(|e| {
              //           let st = Status::from_tonic_status(&e);
              //           return jsonify(&st).unwrap_or(String::from(
              //             "Failed to serialize the error",
              //           ));
              //         })
              //         .unwrap_or_else(|e| e);
              //     })
              //     .map(|txt| Message::text(txt)),
              // );
              let mut stream = resp
                .into_inner()
                .map(|r| {
                  return r
                    .map(|d| {
                      return jsonify(&d).unwrap_or(String::from(
                        "Failed to serialize the progress data.",
                      ));
                    })
                    .map_err(|e| {
                      let st = Status::from_tonic_status(&e);
                      return jsonify(&st).unwrap_or(String::from(
                        "Failed to serialize the error",
                      ));
                    })
                    .unwrap_or_else(|e| e);
                })
                .map(|txt| Message::text(txt));
              while let Some(item) = stream.next().await {
                let _ = sock.send(item).await;
                let _ = sock.flush();
              }
            }
          };
          let _ = sock.close();
        });
      })
      .boxed();
  }
}

#[async_trait]
impl HistChart for Service {
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
          Ok(v) => match v {
            KlineFetchStatus::WIP(p) => p,
            _ => {continue;}
          },
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
    let mut stop_vec = vec![];
    for exc in req.exchanges {
      match FromPrimitive::from_i32(exc) {
        Some(Exchanges::Binance) => {
          stop_vec.push(self.binance.clone().stop());
        }
        _ => {
          continue;
        }
      }
    }
    for result in join_all(stop_vec).await {
      rpc_ret_on_err!(Code::Internal, result);
    }
    return Ok(Response::new(()));
  }
}
