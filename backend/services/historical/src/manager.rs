use ::std::collections::HashMap;

use ::futures::{join, StreamExt};

use ::nats::asynk::{Connection as NatsConnection, Subscription as NatsSubsc};

use ::binance_histories::HistoryFetcher;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::rpc::entities::Exchanges;
use ::rpc::historical::HistChartProg;
use ::slog::{error, o, Logger};
use ::types::ThreadSafeResult;

use crate::entities::KlineFetchStatus;

#[derive(Debug, Clone)]
pub(crate) struct ExchangeManager<T>
where
  T: HistoryFetcher + Send,
{
  pub history_fetcher: T,
  exchange: Exchanges,
  nats: NatsConnection,
  logger: Logger,
}

impl<T> ExchangeManager<T>
where
  T: HistoryFetcher + Send,
{
  pub fn new(
    exchange: Exchanges,
    history_fetcher: T,
    nats: NatsConnection,
    logger: Logger,
  ) -> Self {
    return Self {
      history_fetcher,
      exchange,
      nats,
      logger,
    };
  }
  pub async fn refresh_historical_klines(
    &self,
    symbols: Vec<String>,
  ) -> ThreadSafeResult<()> {
    let mut prog =
      Box::pin(
        self.history_fetcher.refresh(symbols).await?.filter_map(
          |msg| async move {
            from_msgpack::<HistChartProg>(msg.data.as_slice()).ok()
          },
        ),
      );
    let logger_in_thread = self
      .logger
      .new(o!("scope" => "refresh_historical_klines.thread"));
    let nats_con = self.nats.clone();
    let exchange = self.exchange;
    ::tokio::spawn(async move {
      let mut hist_fetch_prog = HashMap::new();
      while let Some(prog) = prog.next().await {
        let result = match hist_fetch_prog.get_mut(&prog.symbol) {
          None => {
            let mut prog_clone = prog.clone();
            prog_clone.cur_symbol_num = (hist_fetch_prog.len() + 1) as i64;
            hist_fetch_prog.insert(prog.symbol.clone(), prog_clone);
            &prog
          }
          Some(v) => {
            v.cur_object_num += prog.cur_object_num;
            v
          }
        };
        let result = KlineFetchStatus::Progress {
          exchange,
          progress: result.to_owned(),
        };
        nats_broadcast_status(&logger_in_thread, &nats_con, &result).await;
      }
    });
    return Ok(());
  }

  pub async fn subscribe(&self) -> ThreadSafeResult<NatsSubsc> {
    return match self.nats.subscribe("kline.progress").await {
      Err(err) => Err(Box::new(err)),
      Ok(v) => Ok(v),
    };
  }

  pub async fn stop(&self) -> ThreadSafeResult<()> {
    let status = KlineFetchStatus::Stop;
    let msg = to_msgpack(&status)?;
    let stop_progress = self.nats.publish("kline.progress", &msg[..]);
    let stop_hist_fetch = self.history_fetcher.stop();
    let (stop_progress, stop_hist_fetch) =
      join!(stop_progress, stop_hist_fetch);
    let _ = stop_progress.or(stop_hist_fetch)?;
    return Ok(());
  }
}

async fn nats_broadcast_status(
  log: &Logger,
  con: &NatsConnection,
  status: &KlineFetchStatus,
) {
  let msg = match to_msgpack(status) {
    Ok(v) => v,
    Err(err) => {
      error!(
        log,
        "Failed to generate a message to broadcast history fetch
        progress: {}, status: {:?}",
        err,
        status,
      );
      return;
    }
  };
  match con.publish("kline.progress", &msg[..]).await {
    Err(err) => {
      error!(
        log,
        "Failed to broadcast history fetch progress: {}, status: {:?}",
        err,
        status,
      );
      return;
    }
    Ok(_) => return,
  }
}
