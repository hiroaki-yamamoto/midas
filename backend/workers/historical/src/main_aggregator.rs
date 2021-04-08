use ::std::collections::HashMap;

use ::clap::Clap;
use ::futures::future::{join_all, select};
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

  let result_st = FetchStatusPubSub::new(broker);
  let mut stop =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  loop {
    select! {
      Some((exchange, part)) = part_stream.next() => {
        let mut prog_map = kvs.entry(exchange).or_insert(HashMap::new());
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
      let _= result_st.publish(&result);
      },
      _ = stop.recv() => {
        let _ = binance_handler.unsubscribe().unwrap();
        part_stream.clear();
        break;
      },
    };
  }
}
