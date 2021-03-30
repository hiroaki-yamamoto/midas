use ::std::fmt::Debug;
use ::std::pin::Pin;

use ::futures::executor::block_on;
use ::futures::future::join_all;
use ::futures::stream::BoxStream;
use ::futures::{SinkExt, Stream, StreamExt};
use ::mongodb::Database;
use ::nats::Connection as NatsCon;
use ::num_traits::FromPrimitive;
use ::serde_json::to_string as jsonify;
use ::slog::{o, Logger};
use ::tokio::select;
use ::warp::filters::BoxedFilter;
use ::warp::ws::{Message, WebSocket, Ws};
use ::warp::{Filter, Reply};

use ::rpc::entities::{Exchanges, Status};
use ::rpc::historical::{HistChartFetchReq, HistChartProg, StopRequest};
use ::types::{GenericResult, ThreadSafeResult};
use binance_histories::fetcher as binance_hist;
use binance_symbols::fetcher as binance_sym;

use super::manager::ExchangeManager;

use super::entities::KlineFetchStatus;

type SubscribeStream<'a> = BoxStream<'a, HistChartProg>;

#[derive(Debug, Clone)]
pub struct Service {
  logger: Logger,
  binance: ExchangeManager<binance_hist::HistoryFetcher>,
}

impl Service {
  pub async fn new(
    log: &Logger,
    db: &Database,
    nats: NatsCon,
  ) -> GenericResult<Self> {
    let log = log.new(o!("scope" => "History Fetch RPC Service"));
    let binance = binance_hist::HistoryFetcher::new(
      None,
      log.new(o!("exchange" => "Binance", "scope" => "HistoryFetch")),
      nats.clone(),
      binance_sym::SymbolFetcher::new(
        log.new(o!("exchange" => "Binance", "scope" => "SymbolFetch")),
        nats.clone(),
        db.clone(),
      )
      .await,
    )
    .await?;
    let binance = ExchangeManager::new(Exchanges::Binance, binance, nats);
    let ret = Self {
      logger: log.clone(),
      binance,
    };
    return Ok(ret);
  }

  pub fn route(&self) -> BoxedFilter<(impl Reply,)> {
    return self
      .sync()
      .or(self.stop())
      .or(self.websocket_subscribe())
      .boxed();
  }

  fn websocket_subscribe(&self) -> BoxedFilter<(impl Reply,)> {
    let ws_svc = self.clone();
    return ::warp::path("subscribe")
      .and(::warp::ws())
      .map(move |ws: Ws| {
        let ws_svc = ws_svc.clone();
        return ws.on_upgrade(|mut sock: WebSocket| async move {
          let subsc = ws_svc.subscribe();
          match subsc {
            Err(e) => {
              let msg = format!(
                "Got an error while trying to subscribe the channel: {}",
                e
              );
              let _ = sock.send(Message::close_with(1011 as u16, msg)).await;
              let _ = sock.flush().await;
            }
            Ok(resp) => {
              let mut stream = resp
                .map(|r| {
                  return jsonify(&r).unwrap_or(String::from(
                    "Failed to serialize the progress data.",
                  ));
                })
                .map(|txt| Message::text(txt));
              loop {
                select! {
                  Some(item) = stream.next() => {
                    let _ = sock.send(item).await;
                    let _ = sock.flush().await;
                  },
                  Some(msg) = sock.next() => {
                    let msg = msg.unwrap_or(::warp::ws::Message::close());
                    if msg.is_close() {
                      break;
                    }
                  }
                }
              }
            }
          };
          let _ = sock.close().await;
        });
      })
      .boxed();
  }

  fn subscribe(&self) -> GenericResult<SubscribeStream> {
    let stream_logger = self.logger.new(o!("scope" => "Stream Logger"));
    let (handler, subscriber) = self.binance.subscribe()?;
    let subscriber = subscriber
      .map(|status| {
        match status {
          KlineFetchStatus::Progress {
            exchange: _,
            progress,
          } => {
            return Some(progress);
          }
          KlineFetchStatus::Stop => {
            handler.unsubscribe();
            return None;
          } // _ => {continue;}
        };
      })
      .filter_map(|o| async { o });
    return Ok(subscriber.boxed() as SubscribeStream);
  }

  fn empty_or_err(&self, res: ThreadSafeResult<()>) -> impl Reply {
    return res.map_or_else(
      |e| -> Box<dyn Reply> {
        let code = ::http::StatusCode::INTERNAL_SERVER_ERROR;
        return Box::new(::warp::reply::with_status(
          ::warp::reply::json(&Status::new(code, format!("{}", e).as_str())),
          code,
        ));
      },
      |_| -> Box<dyn Reply> { Box::new(::warp::reply()) },
    );
  }

  fn sync(&self) -> BoxedFilter<(impl Reply,)> {
    let me = self.clone();
    return ::warp::path("sync")
      .and(::warp::path::param())
      .and(::warp::post())
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
      .and(::warp::post())
      .and(::warp::body::json())
      .map(move |_: Exchanges, req: HistChartFetchReq| {
        let resp = me.empty_or_err(block_on(
          me.binance.refresh_historical_klines(req.symbols),
        ));
        return resp;
      })
      .boxed();
  }

  fn stop(&self) -> BoxedFilter<(impl Reply,)> {
    let me = self.clone();
    return ::warp::path("sync")
      .and(::warp::delete())
      .and(::warp::body::json())
      .map(move |req: StopRequest| {
        let resp = me.empty_or_err(block_on(me.stop_exchanges(req)));
        return resp;
      })
      .boxed();
  }

  async fn stop_exchanges(&self, req: StopRequest) -> ThreadSafeResult<()> {
    let mut stop_vec = vec![];
    for exc in req.exchanges {
      match FromPrimitive::from_i32(exc) {
        Some(Exchanges::Binance) => {
          stop_vec.push(self.binance.stop());
        }
        _ => {
          continue;
        }
      }
    }
    let errs = join_all(stop_vec)
      .await
      .into_iter()
      .filter_map(|f| f.err())
      .nth(0);
    return match errs {
      Some(e) => Err(e),
      None => Ok(()),
    };
  }

  pub async fn graceful_shutdown(&self) -> ThreadSafeResult<()> {
    let _ = self
      .stop_exchanges(StopRequest {
        exchanges: vec![Exchanges::Binance as i32],
      })
      .await?;
    return Ok(());
  }
}
