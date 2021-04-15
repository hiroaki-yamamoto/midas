use ::std::collections::HashMap;

use ::clap::Clap;
use ::futures::StreamExt;
use ::libc::{SIGINT, SIGTERM};
use ::nats::connect;
use ::rpc::entities::Exchanges;
use ::rpc::historical::HistChartProg;
use ::tokio::select;
use ::tokio::signal::unix as signal;
use ::tokio_stream::StreamMap;

use ::binance_histories::pubsub as binance_pubsub;
use ::config::{CmdArgs, Config};
use ::history::{entities::KlineFetchStatus, FetchStatusPubSub};
use ::subscribe::PubSub;

#[tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let logger = cfg.build_slog();
  ::slog::info!(logger, "Kline fetch worker");
  let broker = connect(&cfg.broker_url).unwrap();

  let mut kvs: HashMap<Exchanges, HashMap<String, HistChartProg>> =
    HashMap::new();
  let mut part_stream = StreamMap::new();

  // Binance
  let part = binance_pubsub::HistProgPartPubSub::new(broker.clone());
  let (binance_handler, mut st) = part.queue_subscribe("aggregate").unwrap();
  part_stream.insert(Exchanges::Binance, &mut st);

  let status_pubsub = FetchStatusPubSub::new(broker);
  let (status_handler, mut status_st) = status_pubsub.subscribe().unwrap();
  let mut stop =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  loop {
    select! {
      Some((exchange, part)) = part_stream.next() => {
        let prog_map = kvs.entry(exchange).or_insert(HashMap::new());
        let prev = prog_map.get(&part.symbol).cloned();
        let result = match prog_map.get_mut(&part.symbol) {
          None => {
            let mut prog_clone = part.clone();
            prog_clone.cur_symbol_num = (prog_map.len() + 1) as i64;
            prog_map.insert(part.symbol.clone(), prog_clone);
            &part
          }
          Some(v) => {
            v.cur_object_num += part.cur_object_num;
            v
          }
        };
        let result = KlineFetchStatus::ProgressChanged {
          exchange: Exchanges::Binance,
          previous: prev,
          current: result.clone(),
        };
        let _= status_pubsub.publish(&result);
      },
      Some(status) = status_st.next() => {
        match status {
          KlineFetchStatus::ProgressChanged{
            exchange,
            previous,
            current: remote_current} => {
              let local = kvs.entry(exchange).or_insert(HashMap::new());
              let local_current = local
                .get(&remote_current.symbol);
              let diff = match previous {
                Some(prev) => { &remote_current - &prev },
                None => { Ok(remote_current.clone()) },
              };
              let diff = match diff {
                Err(e) => {
                  ::slog::error!(logger, "Failed to take the diff: {:?}", e);
                  continue;
                },
                Ok(o) => o
              };
              let prog_candidate = match local_current {
                None => {
                  local.insert(remote_current.symbol.clone(), remote_current);
                  continue;
                },
                Some(local_current) => {
                  local_current + &diff
                },
              };
              let prog_candidate = match prog_candidate {
                Err(e) => {
                  ::slog::error!(logger, "Failed to apply the diff: {:?}", e);
                  continue;
                },
                Ok(o) => o,
              };
              if remote_current > prog_candidate {
                local.insert(remote_current.symbol.clone(), remote_current);
              } else {
                local.insert(
                  prog_candidate.symbol.clone(),
                  prog_candidate.clone()
                );
                let _ = status_pubsub.publish(&KlineFetchStatus::ProgressChanged {
                  exchange: Exchanges::Binance,
                  previous: Some(prog_candidate.clone()),
                  current: prog_candidate
                });
              }
          },
          KlineFetchStatus::Done{exchange, symbol} => {
            let _ = kvs.get_mut(&exchange).unwrap_or(&mut HashMap::new()).remove(&symbol);
          },
          _ => {},
        }
      },
      _ = stop.recv() => {
        let _ = binance_handler.unsubscribe().unwrap();
        let _ = status_handler.unsubscribe().unwrap();
        part_stream.clear();
        break;
      },
    };
  }
}
