use ::std::collections::HashMap;
use ::std::pin::Pin;

use ::futures::Stream;

use ::mongodb::Database;
use ::slog::{o, warn, Logger};
use ::tokio::sync::{broadcast, mpsc};
use ::tonic::{async_trait, Code, Request, Response, Status};

use ::types::{rpc_ret_on_err, Result, SendableErrorResult};

use ::rpc::historical::{
  hist_chart_server::HistChart, HistChartFetchReq, HistChartProg,
};

use ::exchanges::{Binance, Exchange};

#[derive(Debug)]
struct ExchangeManager<T>
where
  T: Exchange,
{
  pub exchange: T,
  pub hist_fetch_prog: HashMap<String, HistChartProg>,
}

impl<T> ExchangeManager<T>
where
  T: Exchange,
{
  fn new(exchange: T) -> Self {
    return Self {
      exchange,
      hist_fetch_prog: HashMap::new(),
    };
  }
  async fn refresh_historical_klines(
    &mut self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<(
    broadcast::Sender<()>,
    mpsc::Receiver<SendableErrorResult<HistChartProg>>,
  )> {
    let hist_fut = self.exchange.refresh_historical(symbols);
    return hist_fut.await;
  }

  fn is_completed(&self) -> bool {
    match self.hist_fetch_prog.iter().next() {
      None => return false,
      Some((_, v)) => {
        return v.num_symbols == (self.hist_fetch_prog.len() as i64)
          && self
            .hist_fetch_prog
            .values()
            .all(|item| item.num_objects == item.cur_object_num);
      }
    };
  }
}

#[derive(Debug)]
pub struct Server {
  logger: Logger,
  binance: ExchangeManager<Binance>,
}

impl Server {
  fn new(log: Logger, db: &Database) -> Self {
    return Self {
      logger: log,
      binance: ExchangeManager::new(Binance::new(
        log.new(o!("Exchange" => "Binance")),
        db.collection("binance.history"),
        db.collection("binance.symbolinfo"),
      )),
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
    let manager = self.binance;
    let (stop, progress) = rpc_ret_on_err!(
      Code::Internal,
      manager.refresh_historical_klines(req.symbols).await
    );
    let out = async_stream::try_stream! {
      while !manager.is_completed() {
        let data = match progress.recv().await {
          None => continue,
          Some(d) => match d {
            Err(e) => {
              warn!(self.logger, "Got an error: {}", e);
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
