use ::std::collections::HashMap;

use ::tokio::sync::{broadcast, mpsc};

use ::exchanges::Exchange;
use ::rpc::historical::HistChartProg;
use ::types::SendableErrorResult;

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
