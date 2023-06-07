mod cli;

use std::path::PathBuf;

use crate::cli::Cli;
use clap::Parser;
use coderunner_cli::run_static;
use log::LevelFilter;
use simple_logger::SimpleLogger;

fn main() {
    let args: Cli = Cli::parse();

    let verbosity: LevelFilter = if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };

    // Initializing global logger
    SimpleLogger::new()
        .with_level(verbosity)
        .with_colors(true)
        .without_timestamps()
        .init()
        .unwrap();

    log::info!("Debugging info is turned on by flag (-v, --verbose)");

    log::info!("Running target binary: {}", args.bin.display());

    let full_path: PathBuf = if args.bin.is_absolute() {
        args.bin
    } else {
        args.bin.canonicalize().unwrap()
    };

    run_static(full_path);
}
