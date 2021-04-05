use ::std::collections::HashMap;

use ::clap::Clap;
use ::futures::future::{join_all, select};
use ::libc::{SIGINT, SIGTERM};
use ::nats::connect;
use ::rpc::entities::Exchanges;
use ::rpc::historical::HistChartProg;
use ::tokio::select;
use ::tokio::signal::unix as signal;

use ::binance_histories::pubsub::HistProgPartPubSub;
use ::config::{CmdArgs, Config};
use ::history::FetchStatusPubSub;
use ::subscribe::PubSub;

#[tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let logger = cfg.build_slog();
  ::slog::info!(logger, "Kline fetch worker");
  let broker = connect(&cfg.broker_url).unwrap();

  let kvs: HashMap<Exchanges, HashMap<String, HistChartProg>> = HashMap::new();
  let part = HistProgPartPubSub::new(broker.clone());
  let (part_handler, part) = part.queue_subscribe("aggregate").unwrap();
  let result_st = FetchStatusPubSub::new(broker);
  let mut stop =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  loop {
    select! {
      _ = stop.recv() => {
        let _ = part_handler.unsubscribe().unwrap();
        break;
      },
    };
  }
}
