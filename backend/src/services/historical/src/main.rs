use ::clap::Clap;

#[derive(Clap)]
#[clap(author="Hiroaki Yamamoto")]
struct CmdArgs {
    #[clap(short, long, default_value="/etc/midas/historical.yml")]
    config: String,
}

fn main() {
    let args: CmdArgs = CmdArgs::parse();
    println!("{}", args.config);
}
