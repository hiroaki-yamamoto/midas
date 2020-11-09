use ::clap::Clap;

use ::config::{CmdArgs, Config};
use ::exchanges::binance;

fn main() {
  let cmd: CmdArgs = CmdArgs::parse();
  let cfg = Config::from_fpath(Some(cmd.config)).unwrap();
  let (logger, _) = cfg.build_slog();
  println!("Hello, world!");
}
