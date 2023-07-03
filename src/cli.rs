use clap::Parser;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[arg(short, long, default_value_t = false)]
    pub capture: bool,
    #[arg(short, long, default_value_t = false, conflicts_with = "snapshot")]
    pub replay: bool,
}

impl Arguments {
    pub fn parse_args() -> Arguments {
        Arguments::parse()
    }
}
