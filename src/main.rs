use std::process;

use clap::Parser;
use shell::Config;

fn main() {
    let args = shell::Args::parse();
    let config = Config::build(args);

    if let Err(e) = shell::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
