use aoc_2022::*;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(short, long, default_value_t = 3)]
    day: u8,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let day = cli.day;

    match day {
        1 => day_01::process().await,
        2 => day_02::process().await,
        3 => day_03::process().await,
        _ => {
            println!("Hello, world - {day}!");
        }
    }
}
