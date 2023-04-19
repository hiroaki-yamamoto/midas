#[cfg(debug_assertions)]
use ::std::collections::HashSet;

use ::std::collections::HashMap;

use ::clap::Parser;
use ::futures::StreamExt;
use ::libc::{SIGINT, SIGTERM};
use ::log::{as_error, error, info};

#[cfg(debug_assertions)]
use ::log::{as_serde, warn};

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
  cfg.init_logger();
  info!("Kline fetch worker");
  let broker = cfg.nats_cli().unwrap();
  let db = cfg.db().await.unwrap();
  let redis = cfg.redis().unwrap();
  let mut cur_prog_kvs = CurrentSyncProgressStore::new(redis);

  let pubsub = HistChartPubSub::new(broker.clone());
  let mut sub = pubsub.queue_subscribe("histFetcherWorkerSubsc").unwrap();
  let change_event_pub = FetchStatusEventPubSub::new(broker);
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();

  let mut reg: HashMap<Exchanges, Box<dyn HistoryFetcherTrait>> =
    HashMap::new();

  let fetcher = HistoryFetcher::new(None).unwrap();
  let writer = HistoryWriter::new(&db).await;
  reg.insert(Exchanges::Binance, Box::new(fetcher));

  #[cfg(debug_assertions)]
  let mut dupe_map: HashMap<(Exchanges, String), HashSet<(_, _)>> =
    HashMap::new();

  loop {
    select! {
      Some((req, msg)) = sub.next() => {
        #[cfg(debug_assertions)]
        {
          if let Some(dupe_list) = dupe_map.get_mut(&(req.exchange, req.symbol.clone())) {
            if dupe_list.contains(&(req.start, req.end)) {
              warn!(
                request = as_serde!(req);
                "Dupe detected.",
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
        let klines = match reg.get_mut(&req.exchange) {
          Some(fetcher) => {
            match fetcher.fetch(&req).await {
              Err(e) => {
                error!(error = as_error!(e); "Failed to fetch klines");
                continue;
              },
              Ok(k) => k
            }
          },
          None => {
            error!("Unknown Exchange: {}", req.exchange.as_string());
            continue;
          }
        };
        if let Err(e) = writer.write(klines).await {
          error!(error = as_error!(e); "Failed to write the klines");
          continue;
        }
        let _ = msg.ack();
        if let Err(e) = cur_prog_kvs.incr(
          req.exchange.as_string(),
          req.symbol.clone(), 1
        ) {
          error!(error = as_error!(e); "Failed to report the progress");
        };
        if let Err(e) = change_event_pub.publish(&FetchStatusChanged{
          exchange: req.exchange,
          symbol: req.symbol,
        }) {
          error!(
            error = as_error!(e);
            "Failed to broadcast progress changed event"
          );
        };
      },
      _ = sig.recv() => {
        break;
      },
    }
  }
}
