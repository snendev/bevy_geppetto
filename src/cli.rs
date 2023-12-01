use clap::Parser;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[arg(short, long)]
    pub capture: Option<u64>,
    #[arg(short, long, default_value_t = false, conflicts_with = "capture")]
    pub replay: bool,
}

impl Arguments {
    pub fn parse_args() -> Arguments {
        Arguments::parse()
    }

    pub fn mode<'a>(&'_ self) -> &'a str {
        if self.capture.is_some() {
            "capture"
        } else if self.replay {
            "replay"
        } else {
            "sandbox"
        }
    }
}
