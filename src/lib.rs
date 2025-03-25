use std::path::PathBuf;

#[derive(Debug)]
pub enum ShellAction {
    Disk,
    Type(String),
    Dir,
}

#[derive(Debug)]
pub struct Config {
    pub input_path: PathBuf,
    pub action: ShellAction,
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
