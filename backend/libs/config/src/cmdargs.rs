use ::clap::Parser;

use super::constants::DEFAULT_CONFIG_PATH;

#[derive(Parser)]
#[clap(author = "Hiroaki Yamamoto")]
pub struct CmdArgs {
  #[clap(short, long, default_value = DEFAULT_CONFIG_PATH)]
  pub config: String,
}
