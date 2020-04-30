// use ::binance::general::General;
// use ::binance::api::Binance;
use ::binance::websockets::*;
use ::logger::gen_drain;
use ::slog::*;
use std::sync::atomic::AtomicBool;

fn main() {
  let log = Logger::root(gen_drain(), o!());
  // let api = Binance::new(None, None);
  let atomic_bool = AtomicBool::new(true);
  let mut soc: WebSockets = WebSockets::new(|event: WebsocketEvent| {
    match event {
      WebsocketEvent::DayTicker(ticker_events) => {
        for ticker_evnet in ticker_events {
          info!(
            log,
            "Symbol {}, Price: {}, qty: {}",
            ticker_evnet.symbol, ticker_evnet.best_bid, ticker_evnet.best_bid_qty,
          );
        }
      },
      _ => {},
    }
    return Ok(());
  });
  soc.connect("!ticker@arr").unwrap();
  if let Err(e) = soc.event_loop(&atomic_bool) {
      crit!(log, "Error: {}", e);
      panic!()
  }
  soc.disconnect().unwrap();
  info!(log, "disconnected");
}
