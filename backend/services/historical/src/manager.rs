use ::std::collections::HashMap;
use ::std::io::Result as IOResult;

use ::futures::stream::BoxStream;
use ::futures::StreamExt;

use ::nats::subscription::Handler;
use ::nats::Connection as NatsConnection;

use ::history_fetcher::HistoryFetcher;
use ::rmp_serde::to_vec as to_msgpack;
use ::rpc::entities::Exchanges;
use ::subscribe::to_stream as nats_to_stream;
use ::types::{GenericResult, ThreadSafeResult};

use crate::entities::KlineFetchStatus;

#[derive(Debug, Clone)]
pub(crate) struct ExchangeManager<T>
where
  T: HistoryFetcher + Send + Sync,
{
  pub history_fetcher: T,
  exchange: Exchanges,
  nats: NatsConnection,
}

impl<T> ExchangeManager<T>
where
  T: HistoryFetcher + Send + Sync,
{
  pub fn new(
    exchange: Exchanges,
    history_fetcher: T,
    nats: NatsConnection,
  ) -> Self {
    return Self {
      history_fetcher,
      exchange,
      nats,
    };
  }
  pub async fn refresh_historical_klines(
    &self,
    symbols: Vec<String>,
  ) -> ThreadSafeResult<BoxStream<'_, KlineFetchStatus>> {
    let mut hist_fetch_prog = HashMap::new();
    let exchange = self.exchange.clone();
    let nats_con = self.nats.clone();
    let prog = self
      .history_fetcher
      .refresh(symbols)
      .await?
      .map(move |prog| {
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
        self.nats_broadcast_status(&result);
        return result;
      })
      .boxed();
    return Ok(prog);
  }

  fn nats_broadcast_status(
    &self,
    status: &KlineFetchStatus,
  ) -> GenericResult<()> {
    let msg = to_msgpack(status)?;
    return Ok(self.nats.publish("kline.progress", &msg[..])?);
  }

  pub fn subscribe(
    &self,
  ) -> IOResult<(Handler, BoxStream<'_, KlineFetchStatus>)> {
    let (handler, st) = nats_to_stream::<KlineFetchStatus>(
      self.nats.subscribe("kline.progress")?,
    );
    return Ok((handler, st.boxed()));
  }

  pub async fn stop(&self) -> ThreadSafeResult<()> {
    let status = KlineFetchStatus::Stop;
    let msg = to_msgpack(&status)?;
    let stop_progress = self.nats.publish("kline.progress", &msg[..]);
    let stop_hist_fetch = self.history_fetcher.stop().await;
    let _ = stop_progress.or(stop_hist_fetch)?;
    return Ok(());
  }
}
