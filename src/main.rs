use std::process;

use clap::Parser;
use shell::Config;

/// A shell for the Flinstone Disk Project
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, default_value_t = String::from(""))]
    input_file: String,

    /// Print files
    #[arg(long)]
    dir: bool,

    /// Print contents of file
    #[arg(long, default_value_t = String::from(""))]
    type_: String,
}

fn main() {
    let args = Args::parse();
    let config = Config::build(&args.input_file, args.dir, &args.type_);

    if let Err(e) = shell::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
