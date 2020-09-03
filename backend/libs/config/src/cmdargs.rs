use ::clap::Clap;

use super::constants::DEFAULT_CONFIG_PATH;

#[derive(Clap)]
#[clap(author = "Hiroaki Yamamoto")]
pub struct CmdArgs {
  #[clap(short, long, default_value = DEFAULT_CONFIG_PATH)]
  pub config: String,
}
