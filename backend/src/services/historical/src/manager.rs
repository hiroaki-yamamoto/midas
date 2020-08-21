use ::std::collections::HashMap;
use ::std::thread;

use ::tokio::sync::{broadcast, mpsc};

use ::exchanges::Exchange;
use ::rpc::historical::HistChartProg;
use ::slog::{error, o, Logger};
use ::types::SendableErrorResult;

const CHAN_BUF_SIZE: usize = 1024;

#[derive(Debug)]
pub(crate) struct ExchangeManager<T>
where
  T: Exchange + Send,
{
  pub exchange: T,
  pub hist_fetch_prog: HashMap<String, HistChartProg>,
  logger: Logger,
}

impl<T> ExchangeManager<T>
where
  T: Exchange + Send,
{
  pub fn new(exchange: T, logger: Logger) -> Self {
    return Self {
      exchange,
      hist_fetch_prog: HashMap::new(),
      logger,
    };
  }
  pub async fn refresh_historical_klines(
    &'static mut self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<(
    broadcast::Sender<()>,
    mpsc::Receiver<SendableErrorResult<HistChartProg>>,
  )> {
    let (stop, mut prog) = self.exchange.refresh_historical(symbols).await?;
    let (mut ret_send, ret_recv) = mpsc::channel(CHAN_BUF_SIZE);
    let mut stop_recv = stop.subscribe();
    let logger_in_thread = self
      .logger
      .new(o!("scope" => "refresh_historical_klines.thread"));
    thread::spawn(move || {
      ::tokio::spawn(async move {
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
          match self.hist_fetch_prog.get_mut(&prog.symbol) {
            None => {
              self
                .hist_fetch_prog
                .insert(prog.symbol.clone(), prog.clone());
            }
            Some(v) => {
              v.cur_symbol_num += prog.cur_symbol_num;
              v.cur_object_num += prog.cur_object_num;
            }
          };
          ret_send.send(Ok(prog));
        }
      });
    });
    return Ok((stop, ret_recv));
  }

  pub fn is_completed(&self) -> bool {
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
