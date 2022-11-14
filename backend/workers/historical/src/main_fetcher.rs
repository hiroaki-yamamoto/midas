use ::std::collections::{HashMap, HashSet};

use ::clap::Parser;
use ::futures::StreamExt;
use ::libc::{SIGINT, SIGTERM};
use ::slog::{error, info, warn};
use ::subscribe::PubSub;
use ::tokio::select;
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};
use ::rpc::entities::Exchanges;

use ::history::binance::fetcher::HistoryFetcher;
use ::history::binance::writer::HistoryWriter;
use ::history::entities::FetchStatusChanged;
use ::history::kvs::CurrentSyncProgressStore;
use ::history::pubsub::{FetchStatusEventPubSub, HistChartPubSub};
use ::history::traits::{
  HistoryFetcher as HistoryFetcherTrait, HistoryWriter as HistoryWriterTrait,
  IncrementalStore,
};

#[tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let logger = cfg.build_slog();
  info!(logger, "Kline fetch worker");
  let broker = cfg.nats_cli().unwrap();
  let db = cfg.db().await.unwrap();
  let redis = cfg.redis(&logger).unwrap();
  let mut cur_prog_kvs = CurrentSyncProgressStore::new(redis);

  let pubsub = HistChartPubSub::new(broker.clone());
  let mut sub = pubsub.queue_subscribe("histFetcherWorkerSubsc").unwrap();
  let change_event_pub = FetchStatusEventPubSub::new(broker);
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();

  let mut reg: HashMap<Exchanges, Box<dyn HistoryFetcherTrait>> =
    HashMap::new();

  let fetcher = HistoryFetcher::new(None, logger.clone()).unwrap();
  let writer = HistoryWriter::new(&db).await;
  reg.insert(Exchanges::Binance, Box::new(fetcher));

  #[cfg(debug_assertions)]
  let mut dupe_map: HashMap<(Exchanges, String), HashSet<(_, _)>> =
    HashMap::new();

  loop {
    select! {
      Some((req, _)) = sub.next() => {
        #[cfg(debug_assertions)]
        {
          if let Some(dupe_list) = dupe_map.get_mut(&(req.exchange, req.symbol.clone())) {
            if dupe_list.contains(&(req.start, req.end)) {
              warn!(
                logger,
                "Dupe detected: (Symbol: {:?}, Start: {:?}, End: {:?})",
                req.symbol, req.start, req.end
              );
            } else {
              dupe_list.insert((req.start, req.end));
            }
          } else {
            let mut dupe_list = HashSet::new();
            dupe_list.insert((req.start, req.end));
            dupe_map.insert((req.exchange, req.symbol.clone()), dupe_list);
          }
        }
        let klines = match reg.get(&req.exchange) {
          Some(fetcher) => {
            match fetcher.fetch(&req).await {
              Err(e) => {
                error!(logger, "Failed to fetch klines: {}", e);
                continue;
              },
              Ok(k) => k
            }
          },
          None => {
            error!(logger, "Unknown Exchange: {}", req.exchange.as_string());
            continue;
          }
        };
        if let Err(e) = writer.write(klines).await {
          error!(logger, "Failed to write the klines: {}", e);
          continue;
        }
        if let Err(e) = cur_prog_kvs.incr(
          req.exchange.as_string(),
          req.symbol.clone(), 1
        ) {
          error!(logger, "Failed to report the progress: {}", e);
        };
        if let Err(e) = change_event_pub.publish(&FetchStatusChanged{
          exchange: req.exchange,
          symbol: req.symbol,
        }) {
          error!(logger, "Failed to broadcast progress changed event: {}", e);
        };
      },
      _ = sig.recv() => {
        break;
      },
    }
  }
}
