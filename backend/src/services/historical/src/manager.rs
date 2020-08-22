use ::std::collections::HashMap;
use ::std::error::Error;
use ::std::thread;

use ::nats::Connection as NatsConnection;
use ::tokio::sync::{broadcast, mpsc, oneshot};

use ::config::CHAN_BUF_SIZE;
use ::exchanges::Exchange;
use ::rmp_serde::Serializer as MsgPackSer;
use ::rpc::historical::HistChartProg;
use ::serde::Serialize;
use ::slog::{error, o, Logger};
use ::types::SendableErrorResult;

use crate::entities::KlineFetchStatus;

#[derive(Debug)]
pub(crate) struct ExchangeManager<'nats, T>
where
  T: Exchange + Send,
{
  pub name: String,
  pub exchange: T,
  nats: &'nats NatsConnection,
  logger: Logger,
}

impl<'nats, T> ExchangeManager<'nats, T>
where
  T: Exchange + Send,
{
  pub fn new(
    name: String,
    exchange: T,
    nats: &'nats NatsConnection,
    logger: Logger,
  ) -> Self {
    return Self {
      exchange,
      name,
      nats,
      logger,
    };
  }
  pub async fn refresh_historical_klines(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<(
    broadcast::Sender<()>,
    mpsc::Receiver<SendableErrorResult<HistChartProg>>,
    oneshot::Receiver<()>,
  )> {
    let (stop, mut prog) = self.exchange.refresh_historical(symbols).await?;
    let (send_complete, recv_complete) = oneshot::channel();
    let (mut ret_send, ret_recv) = mpsc::channel(CHAN_BUF_SIZE);
    let mut stop_recv = stop.subscribe();
    let logger_in_thread = self
      .logger
      .new(o!("scope" => "refresh_historical_klines.thread"));
    let nats_con = self.nats.clone();
    let name = self.name.clone();
    thread::spawn(move || {
      ::tokio::spawn(async move {
        let mut hist_fetch_prog = HashMap::new();
        while let Err(_) = stop_recv.try_recv() {
          let prog = match prog.recv().await {
            None => break,
            Some(v) => match v {
              Err(e) => {
                error!(
                  logger_in_thread,
                  "Got an error when getting progress: {}", e
                );
                continue;
              }
              Ok(k) => k,
            },
          };
          let result = match hist_fetch_prog.get_mut(&prog.symbol) {
            None => {
              hist_fetch_prog.insert(prog.symbol.clone(), prog.clone());
              &prog
            }
            Some(v) => {
              v.cur_symbol_num += prog.cur_symbol_num;
              v.cur_object_num += prog.cur_object_num;
              v
            }
          };
          ret_send.send(Ok(result.clone()));
          let result = KlineFetchStatus::WIP(result.to_owned());
          nats_broadcast_status(&logger_in_thread, &nats_con, &name, &result);
        }
        send_complete.send(());
        let result = KlineFetchStatus::Completed;
        nats_broadcast_status(&logger_in_thread, &nats_con, &name, &result);
      });
    });
    return Ok((stop, ret_recv, recv_complete));
  }
}

fn nats_broadcast_status(
  log: &Logger,
  con: &NatsConnection,
  name: &String,
  status: &KlineFetchStatus,
) -> Result<(), Box<dyn Error>> {
  let mut buf: Vec<u8> = Vec::new();
  let msg = match status.serialize(&mut MsgPackSer::new(&mut buf)) {
    Ok(v) => v,
    Err(err) => {
      error!(
        log,
        "Failed to generate a message to broadcast history fetch
                progress: {}, status: {:?}",
        err,
        status,
      );
      return Err(Box::new(err));
    }
  };
  return Ok(con.publish(&format!("{}.kline_progress", name), &buf[..])?);
}
