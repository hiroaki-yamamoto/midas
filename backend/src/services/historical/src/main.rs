mod config;

use crate::config::Config;
use ::clap::Clap;
use ::std::error::Error;

#[derive(Clap)]
#[clap(author="Hiroaki Yamamoto")]
struct CmdArgs {
    #[clap(short, long, default_value="/etc/midas/historical.yml")]
    config: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: CmdArgs = CmdArgs::parse();
    let cfg = Config::from_fpath(args.config)?;
    print!("{:?}", cfg);
    return Ok(());
}
