#[cfg(debug_assertions)]
use ::std::collections::HashSet;

use ::std::collections::HashMap;
use ::std::sync::Arc;
use ::std::time::Duration;

use ::futures::StreamExt;
use ::log::{as_error, error, info};

#[cfg(debug_assertions)]
use ::log::{as_serde, warn};

use ::rpc::exchanges::Exchanges;
use ::tokio::select;

use ::history::binance::fetcher::HistoryFetcher;
use ::history::binance::writer::HistoryWriter;
use ::history::entities::FetchStatusChanged;
use ::history::kvs::CUR_SYNC_PROG_KVS_BUILDER;
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
    let redis = cfg.redis().await.unwrap();
    let cur_prog_kvs = CUR_SYNC_PROG_KVS_BUILDER.build(redis);

    let pubsub = HistChartPubSub::new(&broker).await.unwrap();
    let mut sub = pubsub.pull_subscribe("historyFetchWroker").await.unwrap();
    let change_event_pub = FetchStatusEventPubSub::new(&broker).await.unwrap();

    let mut reg: HashMap<Exchanges, Box<dyn HistoryFetcherTrait>> =
      HashMap::new();

    let fetcher = HistoryFetcher::new(None).unwrap();
    let writer = HistoryWriter::new(&db).await;
    reg.insert(Exchanges::Binance, Box::new(fetcher));

    #[cfg(debug_assertions)]
    let mut dupe_map: HashMap<(Arc<Box<Exchanges>>, Arc<String>), HashSet<(_, _)>> =
      HashMap::new();
    loop {
      select! {
        Some((req, _)) = sub.next() => {
          let exchange = Arc::new(req.exchange.clone());
          let symbol = Arc::new(req.symbol.clone());
          #[cfg(debug_assertions)]
          {
            if let Some(dupe_list) = dupe_map.get_mut(&(exchange.clone(), symbol.clone())) {
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
              dupe_map.insert((exchange.clone(), symbol.clone()), dupe_list);
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
              error!("Unknown Exchange: {}", req.exchange.as_str().to_lowercase());
              continue;
            }
          };
          if let Err(e) = writer.write(klines).await {
            error!(error = as_error!(e); "Failed to write the klines");
            continue;
          }
          if let Err(e) = cur_prog_kvs.incr(
            Arc::new(exchange.as_str().to_lowercase()),
            symbol.clone(), 1,
            WriteOption::default().duration(Duration::from_secs(180).into()).into()
          ).await {
            error!(error = as_error!(e); "Failed to report the progress");
          };
          if let Err(e) = change_event_pub.publish(&FetchStatusChanged{
            exchange: req.exchange.as_ref().clone(),
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
