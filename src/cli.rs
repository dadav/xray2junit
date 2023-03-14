use std::fs::File;
use std::io;
use std::path::PathBuf;

use clap::Parser;
use tracing::{debug, Level};
use tracing_subscriber;

use crate::format::SimpleJsonResults;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// File to be processed
    #[arg(short, long, value_name = "FILE or STDIN(-)", default_value = "-")]
    input: PathBuf,

    /// File to save the result
    #[arg(short, long, value_name = "FILE or STDOUT(-)", default_value = "-")]
    output: PathBuf,

    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,
}

pub fn run() {
    let config = Cli::parse();

    if config.debug {
        tracing_subscriber::fmt()
            .with_writer(io::stderr)
            .with_max_level(Level::DEBUG)
            .init();
        debug!("Debug mode.");
    }

    let in_file: Box<dyn io::Read> = if config.input.display().to_string() == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(config.input).unwrap())
    };

    let data = SimpleJsonResults::new(in_file).expect("Could not deserialze json");
    debug!("Parsed json: {:?}", data);

    let out_file: Box<dyn io::Write> = if config.output.display().to_string() == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(&config.output).unwrap())
    };

    data.export(out_file).unwrap();
    debug!("Wrote data to {:?}", config.output.display());
}
