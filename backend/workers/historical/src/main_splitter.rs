use ::clap::Parser;
use ::nats::connect as con_nats;

use ::config::{CmdArgs, Config};
use ::history::pubsub::{HistChartPubSub, RawHistChartPubSub};
use subscribe::PubSub;

#[tokio::main]
async fn main() {
  let args: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(args.config)).unwrap();

  let broker = con_nats(&cfg.broker_url).unwrap();
  let req_pubsub = RawHistChartPubSub::new(broker.clone());
  let req_sub = req_pubsub.queue_subscribe("histChartDateSplitter");
  let resp_pubsub = HistChartPubSub::new(broker);
}
