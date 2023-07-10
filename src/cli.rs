use clap::Parser;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[arg(short, long, default_value_t = false)]
    pub capture: bool,
    #[arg(short, long, default_value_t = false, conflicts_with = "capture")]
    pub replay: bool,
}

impl Arguments {
    pub fn parse_args() -> Arguments {
        Arguments::parse()
    }

    pub fn mode<'a>(&'_ self) -> &'a str {
        if self.capture {
            "capture"
        } else if self.replay {
            "replay"
        } else {
            "sandbox"
        }
    }
}
