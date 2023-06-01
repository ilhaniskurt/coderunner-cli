mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let args: Cli = Cli::parse();

    println!("Binary {}", args.bin.display());
}
