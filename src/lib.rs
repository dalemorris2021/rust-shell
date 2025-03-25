use std::{error::Error, fmt::Debug, path::PathBuf};

#[derive(Debug)]
pub enum ShellAction {
    Disk,
    Type(String),
    Dir,
}

#[derive(Debug)]
pub struct Config {
    pub input_path: Option<PathBuf>,
    pub action: ShellAction,
}

impl Config {
    pub fn build(input_file: &str, dir: bool, type_: &str) -> Self {
        let input_path = if input_file == "" {
            None
        } else {
            Some(PathBuf::from(input_file))
        };

        let action = if dir {
            ShellAction::Dir
        } else if type_ != "" {
            ShellAction::Type(String::from(type_))
        } else {
            ShellAction::Disk
        };

        Config { input_path, action }
    }
}

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.input_path {
        None => println!("stdin"),
        Some(path) => println!("{}", path.display()),
    };

    match config.action {
        ShellAction::Disk => println!("disk"),
        ShellAction::Type(type_) => println!("type: {type_}"),
        ShellAction::Dir => println!("dir"),
    };

    Ok(())
}
