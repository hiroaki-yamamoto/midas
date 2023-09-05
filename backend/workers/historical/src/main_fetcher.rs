#[cfg(debug_assertions)]
use ::std::collections::HashSet;

use ::std::collections::HashMap;
use ::std::time::Duration;

use ::futures::StreamExt;
use ::log::{as_error, error, info};

#[cfg(debug_assertions)]
use ::log::{as_serde, warn};

use ::rpc::entities::Exchanges;
use ::tokio::select;

use ::history::binance::fetcher::HistoryFetcher;
use ::history::binance::writer::HistoryWriter;
use ::history::entities::FetchStatusChanged;
use ::history::kvs::CurrentSyncProgressStore;
use ::history::pubsub::{FetchStatusEventPubSub, HistChartPubSub};
use ::history::traits::{
  HistoryFetcher as HistoryFetcherTrait, HistoryWriter as HistoryWriterTrait,
};
use ::kvs::traits::symbol::Incr;
use ::kvs::WriteOption;
use ::subscribe::PubSub;

#[tokio::main]
async fn main() {
  info!("Starting kline fetch worker");
  ::config::init(|cfg, mut sig, db, broker, _| async move {
    let redis = cfg.redis().unwrap();
    let cur_prog_kvs = CurrentSyncProgressStore::new(redis);

    let pubsub = HistChartPubSub::new(&broker).await.unwrap();
    let mut sub = pubsub.pull_subscribe("historyFetchWroker").await.unwrap();
    let change_event_pub = FetchStatusEventPubSub::new(&broker).await.unwrap();

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
        Some((req, _)) = sub.next() => {
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
              error!("Unknown Exchange: {}", req.exchange.as_str_name().to_lowercase());
              continue;
            }
          };
          if let Err(e) = writer.write(klines).await {
            error!(error = as_error!(e); "Failed to write the klines");
            continue;
          }
          if let Err(e) = cur_prog_kvs.incr(
            req.exchange.as_str_name().to_lowercase(),
            req.symbol.clone(), 1,
            WriteOption::default().duration(Duration::from_secs(180).into()).into()
          ).await {
            error!(error = as_error!(e); "Failed to report the progress");
          };
          if let Err(e) = change_event_pub.publish(&FetchStatusChanged{
            exchange: req.exchange,
            symbol: req.symbol,
          }).await {
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
  }).await;
  info!("Stopping kline fetch worker");
}
