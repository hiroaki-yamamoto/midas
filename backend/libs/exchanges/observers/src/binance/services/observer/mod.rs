mod impl_;
mod impl_itrade_observer;

use ::std::collections::HashMap;
use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::rug::Float;
use ::tokio::signal::unix::Signal;

use ::errors::ObserverResult;
use ::subscribe::PubSub;
use ::symbols::entities::SymbolEvent;
use ::symbols::traits::SymbolReader;

use crate::binance::{entities::BookTicker, sockets::BookTickerSocket};
use crate::traits::ITradeObserver;

pub struct TradeObserver {
  pubsub: Arc<dyn PubSub<Output = BookTicker<Float>> + Send + Sync>,
  sockets: HashMap<usize, BookTickerSocket>,
  symbol_reader: Arc<dyn SymbolReader + Send + Sync>,
  symbol_event: Arc<dyn PubSub<Output = SymbolEvent> + Send + Sync>,
}
