use ::std::time::SystemTime;

use ::async_trait::async_trait;
use ::entities::HistoryFetchRequest;

use ::types::ThreadSafeResult;

use crate::entities::KlinesByExchange;

#[async_trait]
pub trait HistoryFetcher {
  // type Kline: Kline;
  async fn fetch(
    &mut self,
    req: &HistoryFetchRequest,
  ) -> ThreadSafeResult<KlinesByExchange>;
  async fn first_trade_date(
    &mut self,
    symbol: &str,
  ) -> ThreadSafeResult<SystemTime>;
}
