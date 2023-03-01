#[cfg(debug_assertions)]
use ::std::collections::HashSet;

use ::std::time::{Duration, UNIX_EPOCH};

use ::clap::Parser;
use ::futures::StreamExt;
use ::libc::{SIGINT, SIGTERM};
use ::log::{as_debug, as_error, error, info, warn};
use ::tokio::select;
use ::tokio::signal::unix as signal;

use ::config::{CmdArgs, Config};
use ::date_splitter::DateSplitter;
use ::history::binance::fetcher::HistoryFetcher as BinanceHistFetcher;
use ::history::binance::writer::HistoryWriter as BinanceHistoryWriter;
use ::history::kvs::{CurrentSyncProgressStore, NumObjectsToFetchStore};
use ::history::pubsub::{HistChartDateSplitPubSub, HistChartPubSub};
use ::history::traits::{
  HistoryFetcher as HistFetchTrait, HistoryWriter as HistoryWriterTrait,
  IncrementalStore as IncrStoreTrait, Store as StoreTrait,
};
use ::rpc::entities::Exchanges;
use ::subscribe::PubSub;

#[tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  cfg.init_logger();
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();

  let mut cur_prog_kvs = CurrentSyncProgressStore::new(cfg.redis().unwrap());
  let mut num_prg_kvs = NumObjectsToFetchStore::new(cfg.redis().unwrap());
  let broker = cfg.nats_cli().unwrap();
  let db = cfg.db().await.unwrap();

  let req_pubsub = HistChartDateSplitPubSub::new(broker.clone());
  let mut req_sub = req_pubsub.queue_subscribe("dateSplitSub").unwrap();
  let resp_pubsub = HistChartPubSub::new(broker.clone());

  info!("Ready.");
  loop {
    select! {
      Some((req, _)) = req_sub.next() => {
        let mut start = req.start.map(|start| start.into()).unwrap_or(UNIX_EPOCH);
        let end = req.end.map(|end| end.into()).unwrap_or(UNIX_EPOCH);
        info!(
          symbol = req.symbol,
          start_at = as_debug!(start),
          end_at = as_debug!(end);
          "Start splitting currency",
        );
        let (fetcher, writer) = match req.exchange {
          Exchanges::Binance => (
            BinanceHistFetcher::new(None),
            BinanceHistoryWriter::new(&db).await,
          ),
        };
        if let Err(e) = writer.delete_by_symbol(&req.symbol).await {
          error!(
            symbol = req.symbol,
            error = as_error!(e);
            "Failed to clean historical data",
          );
          continue;
        };
        if let Ok(mut fetcher) = fetcher {
          start = fetcher.first_trade_date(&req.symbol).await.unwrap_or(start);
        }
        let splitter = match req.exchange {
          Exchanges::Binance => DateSplitter::new(
            start, end, Duration::from_secs(60000)
          ),
        };
        let mut splitter = match splitter {
          Err(e) => {
            error!(error = as_error!(e); "Failed to initialize DateSplitter");
            continue;
          },
          Ok(v) => v
        };
        if let Err(e) = cur_prog_kvs.reset(req.exchange.as_string(), &req.symbol) {
          error!(error = as_error!(e); "Failed to reset the progress");
          continue;
        }
        if let Err(e) = num_prg_kvs.set(
          req.exchange.as_string(),
          &req.symbol,
          splitter.len().unwrap_or(0) as i64
        ) {
          error!(error = as_error!(e); "Failed to set the number of objects to fetch");
        }

        #[cfg(debug_assertions)]
        let mut dupe_list: HashSet<_> = HashSet::new();

        while let Some((start, end)) = splitter.next().await {

          #[cfg(debug_assertions)]
          {
            if dupe_list.contains(&start) {
              warn!(
                start = as_debug!(start),
                end = as_debug!(end);
                "Dupe detected",
              );
            }
          }

          if let Err(e) = resp_pubsub.publish(
            &req.clone().start(Some(start.into())).end(Some(end.into()))
          ) {
            error!(error = as_error!(e); "Error occured while sending splite date data");
          }
        }

        #[cfg(debug_assertions)]
        dupe_list.insert(start);

      },
      _ = sig.recv() => {break;},
    }
  }
}
