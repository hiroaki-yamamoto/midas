use ::std::collections::HashMap;
use ::std::io::Result as IOResult;

use ::futures::stream::BoxStream;
use ::futures::StreamExt;
use ::tokio::sync::oneshot::channel;

use ::nats::subscription::Handler;
use ::nats::Connection as NatsConnection;

use ::history::HistoryFetcher;
use ::rmp_serde::to_vec as to_msgpack;
use ::rpc::entities::Exchanges;
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
  T: HistoryFetcher + Send + Sync + Clone,
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
  ) -> ThreadSafeResult<()> {
    self.history_fetcher.refresh(symbols).await?;
    let exchange = self.exchange.clone();
    let (_, prog_stream) = self.history_fetcher.subscribe_progress()?;
    let mut prog_map = HashMap::new();
    let prog_stream = prog_stream
      .map(move |prog| {
        let result = match prog_map.get_mut(&prog.symbol) {
          None => {
            let mut prog_clone = prog.clone();
            prog_clone.cur_symbol_num = (prog_map.len() + 1) as i64;
            prog_map.insert(prog.symbol.clone(), prog_clone);
            &prog
          }
          Some(v) => {
            v.cur_object_num += prog.cur_object_num;
            v
          }
        };
        let result = KlineFetchStatus::Progress {
          exchange: Exchanges::Binance,
          progress: result.clone(),
        };
        return result;
      })
      .boxed();
    let (st_send, st_recv) = channel();
    st_send.send((self.nats.clone(), prog_stream));
    ::tokio::spawn(async move {
      if let Ok((nats, st)) = st_recv.await {
        // while let Some(prog) = st.next().await {
        //   if let Ok(msg) = to_msgpack(&prog) {
        //     nats.publish("kline.progress", &msg[..]);
        //   }
        // }
      }
    });
    return Ok(());
  }

  pub fn subscribe(
    &self,
  ) -> IOResult<(Handler, BoxStream<'_, KlineFetchStatus>)> {
    let exchange = self.exchange.clone();
    let mut prog_map = HashMap::new();
    let (handler, st) = self.history_fetcher.subscribe_progress()?;
    let prog_stream = st
      .map(move |prog| {
        let result = match prog_map.get_mut(&prog.symbol) {
          None => {
            let mut prog_clone = prog.clone();
            prog_clone.cur_symbol_num = (prog_map.len() + 1) as i64;
            prog_map.insert(prog.symbol.clone(), prog_clone);
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
    return Ok((handler, prog_stream));
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
