use ::clap::Clap;
use ::futures::{FutureExt, SinkExt};
use ::warp::Filter;

use ::config::{CmdArgs, Config};
use ::exchanges::binance;

fn main() {
  let cmd: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(cmd.config)).unwrap();
  let (logger, _) = cfg.build_slog();
  let route = warp::path::param::<String>().and(::warp::ws()).map(
    |exchange: String, ws: ::warp::ws::Ws| {
      return ws.on_upgrade(|socket: ::warp::ws::WebSocket| async {});
    },
  );
  println!("Hello, world!");
}
