use std::{
    error::Error,
    fmt::Debug,
    fs,
    io::{stdin, Read},
    path::PathBuf,
};

use clap::Parser;

/// A shell for the Flinstone Disk Project
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input file
    #[arg(short, default_value_t = String::from(""))]
    pub input_file: String,

    /// Print files
    #[arg(long)]
    pub dir: bool,

    /// Print contents of file
    #[arg(long, default_value_t = String::from(""))]
    pub type_: String,
}

/// A command for the shell to perform
#[derive(Debug)]
pub enum ShellAction {
    Disk,
    Type(String),
    Dir,
}

/// A configuration for the shell
#[derive(Debug)]
pub struct Config {
    pub input_path: Option<PathBuf>,
    pub action: ShellAction,
}

impl Config {
    pub fn build(args: Args) -> Self {
        let input_path = if args.input_file == "" {
            None
        } else {
            Some(PathBuf::from(args.input_file))
        };

        let action = if args.dir {
            ShellAction::Dir
        } else if args.type_ != "" {
            ShellAction::Type(String::from(args.type_))
        } else {
            ShellAction::Disk
        };

        Config { input_path, action }
    }
}

/// A disk cluster
#[derive(Debug)]
pub enum Cluster {
    EmptyCluster {
        next_empty: usize,
    },
    DamagedCluster {
        next_damaged: usize,
    },
    FileDataCluster {
        content: String,
        next_data: usize,
    },
    FileHeaderCluster {
        name: String,
        content: String,
        next_header: usize,
        next_data: usize,
    },
    RootCluster {
        name: String,
        empty: usize,
        damaged: usize,
        headers: usize,
    },
}

/// The start of the shell
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();

    let disk = match config.input_path {
        Some(path) => fs::read_to_string(path)?,
        None => {
            stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    println!("{disk}");

    match config.action {
        ShellAction::Disk => println!("disk"),
        ShellAction::Type(type_) => println!("type: {type_}"),
        ShellAction::Dir => println!("dir"),
    };

    Ok(())
}
