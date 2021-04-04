use ::std::collections::HashMap;

use ::clap::Clap;
use ::futures::future::{join_all, select};
use ::libc::{SIGINT, SIGTERM};
use ::nats::connect;
use ::rpc::entities::Exchanges;
use ::rpc::historical::HistChartProg;
use ::slog::{info, o};
use ::tokio::signal::unix as signal;

use ::binance_histories::pubsub::HistProgPartPubSub;
use ::config::{CmdArgs, Config};
use ::history::pubsub::FetchStatusPubSub;

#[tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();
  let logger = cfg.build_slog();
  info!(logger, "Kline fetch worker");
  let broker = connect(&cfg.broker_url).unwrap();

  let kvs: HashMap<Exchanges, HashMap<String, HistChartProg>> = HashMap::new();
}
