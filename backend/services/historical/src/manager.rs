use ::std::collections::HashMap;

use ::nats::{Connection as NatsConnection, Subscription as NatsSubsc};

use ::exchanges::HistoryFetcher;
use ::rmp_serde::to_vec;
use ::slog::{error, o, Logger};
use ::types::{GenericResult, SendableErrorResult};

use crate::entities::KlineFetchStatus;

#[derive(Debug)]
pub(crate) struct ExchangeManager<'nats, T>
where
  T: HistoryFetcher + Send,
{
  pub name: String,
  pub history_fetcher: &'nats T,
  nats: &'nats NatsConnection,
  logger: Logger,
}

impl<'nats, T> ExchangeManager<'nats, T>
where
  T: HistoryFetcher + Send,
{
  pub fn new(
    name: String,
    history_fetcher: &'nats T,
    nats: &'nats NatsConnection,
    logger: Logger,
  ) -> Self {
    return Self {
      history_fetcher,
      name,
      nats,
      logger,
    };
  }
  pub async fn refresh_historical_klines(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<()> {
    let prog = self.history_fetcher.refresh(symbols).await?;
    let logger_in_thread = self
      .logger
      .new(o!("scope" => "refresh_historical_klines.thread"));
    let nats_con = self.nats.clone();
    let name = self.name.clone();
    ::tokio::spawn(async move {
      let mut hist_fetch_prog = HashMap::new();
      while let Ok(prog) = prog.recv() {
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
        let result = KlineFetchStatus::WIP(result.to_owned());
        nats_broadcast_status(&logger_in_thread, &nats_con, &name, &result);
      }
      let result = KlineFetchStatus::Completed;
      nats_broadcast_status(&logger_in_thread, &nats_con, &name, &result);
    });
    return Ok(());
  }

  pub fn subscribe(&self) -> GenericResult<NatsSubsc> {
    let channel = format!("{}.kline.progress", self.name);
    return match self.nats.subscribe(&channel) {
      Err(err) => Err(Box::new(err)),
      Ok(v) => Ok(v),
    };
  }
}

fn nats_broadcast_status(
  log: &Logger,
  con: &NatsConnection,
  name: &str,
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
  match con.publish(&format!("{}.kline.progress", name), &msg[..]) {
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
