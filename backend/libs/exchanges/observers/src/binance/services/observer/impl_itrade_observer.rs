use ::std::collections::HashSet;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::StreamExt;
use ::tokio::select;
use ::tokio::signal::unix::Signal;
use ::tokio::time::interval;

use ::errors::ObserverResult;
use ::rpc::exchanges::Exchanges;
use ::symbols::entities::SymbolEvent;

use crate::traits::ITradeObserver;

use super::TradeObserver;

#[async_trait]
impl ITradeObserver for TradeObserver {
  async fn start(&mut self, signal: &mut Signal) -> ObserverResult<()> {
    // Pre-subscription symbols from symbol event
    let symbol_event = self.symbol_event.clone();
    let mut symbol_event =
      symbol_event.pull_subscribe("trade_observer").await?;
    let mut sub_unsub_handle_interval = interval(Duration::from_secs(60));
    let mut symbols_to_add = Vec::<String>::new();
    let mut symbols_to_del = Vec::<String>::new();

    // Subscribe all trading sumbols
    let all_symbols: HashSet<String> = self
      .symbol_reader
      .list_trading()
      .await?
      .map(|info| info.symbol)
      .collect()
      .await;
    let all_symbols: Vec<String> = all_symbols.into_iter().collect();
    self.subscribe(&all_symbols).await?;

    let mut call_subscribe;
    let mut call_unsubscribe;
    // Start Event loop
    loop {
      call_subscribe = false;
      call_unsubscribe = false;
      select! {
        _ = signal.recv() => {
          break;
        }
        Some((symbol_event, _)) = symbol_event.next() => {
          match symbol_event {
            SymbolEvent::Add(symbol) => {
              if symbol.exchange == Exchanges::Binance.into() {
                symbols_to_add.push(symbol.symbol);
              }
              if symbols_to_add.len() > 10 {
                call_subscribe = true;
              }
            }
            SymbolEvent::Remove(symbol) => {
              if symbol.exchange == Exchanges::Binance.into() {
                symbols_to_del.push(symbol.symbol);
              }
              if symbols_to_del.len() > 10 {
                call_unsubscribe = true;
              }
            }
          }
        },
        _ = sub_unsub_handle_interval.tick() => {
          self.resubscribe().await?;
          call_subscribe = true;
          call_unsubscribe = true;
        },
        Some((_, payload)) = self.sockets.next() => {
          self.pubsub.publish(&payload).await?;
        }
      }
      if call_subscribe && !symbols_to_add.is_empty() {
        let symbols: Vec<String> = symbols_to_add.drain(..).collect();
        self.subscribe(&symbols).await?;
      }
      if call_unsubscribe && !symbols_to_del.is_empty() {
        let symbols: Vec<String> = symbols_to_del.drain(..).collect();
        self.unsubscribe(&symbols).await?;
      }
    }
    return Ok(());
  }
}
