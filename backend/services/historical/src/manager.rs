use ::nats::Connection as NatsConnection;

use ::history::traits::HistoryFetcher;
use ::rmp_serde::to_vec as to_msgpack;
use ::rpc::entities::Exchanges;
use ::types::ThreadSafeResult;

use history::entities::KlineFetchStatus;

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
    return Ok(());
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
