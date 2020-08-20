use ::std::collections::HashMap;
use ::std::thread;

use ::tokio::sync::{broadcast, mpsc};

use ::exchanges::Exchange;
use ::rpc::historical::HistChartProg;
use ::types::SendableErrorResult;

const CHAN_BUF_SIZE: usize = 1024;

#[derive(Debug)]
pub(crate) struct ExchangeManager<T>
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
  pub fn new(exchange: T) -> Self {
    return Self {
      exchange,
      hist_fetch_prog: HashMap::new(),
    };
  }
  pub async fn refresh_historical_klines(
    &mut self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<(
    broadcast::Sender<()>,
    mpsc::Receiver<SendableErrorResult<HistChartProg>>,
  )> {
    let (stop, mut prog) = self.exchange.refresh_historical(symbols).await?;
    let (ret_send, ret_recv) = mpsc::channel(CHAN_BUF_SIZE);
    let mut stop_recv = stop.subscribe();
    thread::spawn(move || {
      ::tokio::spawn(async move {
        while let Err(_) = stop_recv.try_recv() {
          let prog = match prog.recv().await {
            None => break,
            Some(v) => v,
          };
          ret_send.send(prog);
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
