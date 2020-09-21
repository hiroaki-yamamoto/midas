use ::std::collections::HashMap;

use ::futures::join;

use ::nats::asynk::{Connection as NatsConnection, Subscription as NatsSubsc};

use ::exchanges::HistoryFetcher;
use ::rmp_serde::to_vec;
use ::rpc::entities::Exchanges;
use ::slog::{error, o, Logger};
use ::types::{ret_on_err, GenericResult, SendableErrorResult};

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
  ) -> SendableErrorResult<()> {
    let mut prog = self.history_fetcher.refresh(symbols).await?;
    let logger_in_thread = self
      .logger
      .new(o!("scope" => "refresh_historical_klines.thread"));
    let nats_con = self.nats.clone();
    let exchange = self.exchange;
    ::tokio::spawn(async move {
      let mut hist_fetch_prog = HashMap::new();
      while let Some(prog) = prog.recv().await {
        let prog = match prog {
          Err(e) => {
            error!(
              logger_in_thread,
              "Got an error when getting progress: {}", e
            );
            continue;
          }
          Ok(k) => k,
        };
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

  pub async fn subscribe(&self) -> GenericResult<NatsSubsc> {
    return match self.nats.subscribe("kline.progress").await {
      Err(err) => Err(Box::new(err)),
      Ok(v) => Ok(v),
    };
  }

  pub async fn stop(&self) -> SendableErrorResult<()> {
    let status = KlineFetchStatus::Stop;
    let msg = ret_on_err!(to_vec(&status));
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
  let msg = match to_vec(status) {
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
