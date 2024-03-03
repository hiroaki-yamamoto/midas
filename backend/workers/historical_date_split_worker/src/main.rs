#[cfg(debug_assertions)]
use ::std::collections::HashSet;

use ::std::sync::Arc;

use ::std::time::{Duration, UNIX_EPOCH};

use ::futures::StreamExt;
use ::log::{error, info};

#[cfg(debug_assertions)]
use ::log::warn;

use ::date_splitter::DateSplitter;
use ::history::binance::fetcher::HistoryFetcher as BinanceHistFetcher;
use ::history::binance::writer::HistoryWriter as BinanceHistoryWriter;
use ::history::kvs::{CUR_SYNC_PROG_KVS_BUILDER, NUM_TO_FETCH_KVS_BUILDER};
use ::tokio::{join, select};

use ::history::pubsub::{HistChartDateSplitPubSub, HistChartPubSub};
use ::history::traits::{
  HistoryFetcher as HistFetchTrait, HistoryWriter as HistoryWriterTrait,
};
use ::kvs::traits::symbol::{Incr, Set};
use ::kvs::WriteOption;
use ::rpc::exchanges::Exchanges;
use ::subscribe::PubSub;

#[tokio::main]
async fn main() {
  info!("Starting kline date split worker");
  ::config::init(|cfg, mut sig, db, broker, _| async move {
    let redis = cfg.redis().await.unwrap();
    let cur_prog_kvs = CUR_SYNC_PROG_KVS_BUILDER.build(redis.clone());
    let num_prg_kvs = NUM_TO_FETCH_KVS_BUILDER.build(redis);

    let (req_pubsub, resp_pubsub) = join!(
      HistChartDateSplitPubSub::new(&broker),
      HistChartPubSub::new(&broker),
    );
    let (req_pubsub, resp_pubsub) = (req_pubsub.unwrap(), resp_pubsub.unwrap());
    let mut req_sub = req_pubsub
      .pull_subscribe("historyDateSplitWorker")
      .await
      .unwrap();

    loop {
      select! {
        Some((req, _)) = req_sub.next() => {
          let exchange_name = Arc::new(req.exchange.as_str().to_lowercase());
          let symbol = Arc::new(req.symbol.clone());
          let mut start = req.start.map(|start| start.into()).unwrap_or(UNIX_EPOCH);
          let end = req.end.map(|end| end.into()).unwrap_or(UNIX_EPOCH);
          info!(
            symbol = req.symbol,
            start_at: debug = start,
            end_at: debug = end;
            "Start splitting currency",
          );
          let (fetcher, writer) = match req.exchange.as_ref() {
            Exchanges::Binance => (
              BinanceHistFetcher::new(None),
              BinanceHistoryWriter::new(&db).await,
            ),
          };
          if let Err(e) = writer.delete_by_symbol(&req.symbol).await {
            error!(
              symbol = req.symbol,
              error: err = e;
              "Failed to clean historical data",
            );
            continue;
          };
          if let Ok(mut fetcher) = fetcher {
            start = fetcher.first_trade_date(&req.symbol).await.unwrap_or(start);
          }
          let splitter = match req.exchange.as_ref() {
            Exchanges::Binance => DateSplitter::new(
              start, end, Duration::from_secs(60000)
            ),
          };
          let mut splitter = match splitter {
            Err(e) => {
              error!(error: err = e; "Failed to initialize DateSplitter");
              continue;
            },
            Ok(v) => v
          };
          if let Err(e) = cur_prog_kvs.reset(
            exchange_name.clone(), symbol.clone()
          ).await {
            error!(error: err = e; "Failed to reset the progress");
            continue;
          }
          if let Err(e) = num_prg_kvs.set(
            exchange_name.clone(),
            symbol.clone(),
            splitter.len().unwrap_or(0) as i64,
            WriteOption::default().duration(Duration::from_secs(180).into()).into(),
          ).await {
            error!(error: err = e; "Failed to set the number of objects to fetch");
          }

          #[cfg(debug_assertions)]
          let mut dupe_list: HashSet<_> = HashSet::new();

          while let Some((start, end)) = splitter.next().await {

            #[cfg(debug_assertions)]
            {
              if dupe_list.contains(&start) {
                warn!(
                  start: debug = start,
                  end: debug = end;
                  "Dupe detected",
                );
              }
            }

            if let Err(e) = resp_pubsub.publish(
              &req.clone().start(Some(start.into())).end(Some(end.into()))
            ).await {
              error!(
                error: err = e;
                "Error occured while sending splite date data"
              );
            }
          }

          #[cfg(debug_assertions)]
          dupe_list.insert(start);

        },
        _ = sig.recv() => {break;},
      }
    }
  }).await;
  info!("Stopping kline date split worker");
}
