use ::std::io::Result as IOResult;

use ::async_trait::async_trait;
use ::nats::subscription::Handler;
use ::nats::Message;

use ::rpc::historical::HistChartProg;
use ::types::ThreadSafeResult;

#[async_trait]
pub trait HistoryFetcher {
  async fn refresh(&self, symbols: Vec<String>) -> ThreadSafeResult<()>;
  async fn stop(&self) -> ThreadSafeResult<()>;
  async fn spawn(&self) -> ThreadSafeResult<()>;
  fn subscribe_progress<T>(&self, func: T) -> IOResult<Handler>
  where
    T: Fn(HistChartProg, Message) -> IOResult<()> + Send + 'static;
}
