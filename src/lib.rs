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
    Root {
        name: String,
        empty: usize,
        damaged: usize,
        headers: usize,
    },
    Empty {
        next_empty: usize,
    },
    Damaged {
        next_damaged: usize,
    },
    FileHeader {
        name: String,
        content: String,
        next_header: usize,
        next_data: usize,
    },
    FileData {
        content: String,
        next_data: usize,
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

    let clusters = disk_to_clusters(&disk)?;

    match config.action {
        ShellAction::Disk => print_disk(&clusters),
        ShellAction::Type(type_) => println!("type: {type_}"),
        ShellAction::Dir => print_files(&clusters),
    };

    Ok(())
}

fn disk_to_clusters(disk: &str) -> Result<Vec<Cluster>, Box<dyn Error>> {
    let trimmed_disk = format_disk(disk);
    let clusters = trimmed_disk
        .iter()
        .map(|raw| raw_to_cluster(raw))
        .collect::<Result<Vec<Cluster>, _>>()?;

    Ok(clusters)
}

fn format_disk(disk: &str) -> Vec<Vec<u8>> {
    let mut bytes: Vec<Vec<u8>> = Vec::new();
    let mut line = String::new();

    let lines: Vec<&str> = disk.lines().collect();
    for (i, l) in lines.iter().enumerate() {
        if i >= 2 && i < l.len() {
            let mut byte_line: Vec<u8> = Vec::new();
            line.push_str("0");
            line.push_str(&l[3..l.len()]);

            let mut sum = 0;
            for (j, char) in line.chars().enumerate() {
                if j % 2 == 0 {
                    sum = 0x10 as u8 * char::to_digit(char, 0x10).unwrap() as u8;
                } else {
                    sum += char::to_digit(char, 0x10).unwrap() as u8;
                    byte_line.push(sum);
                }
            }
            bytes.push(byte_line);

            line.clear();
        }
    }

    bytes
}

fn raw_to_cluster(raw: &[u8]) -> Result<Cluster, &'static str> {
    let num_bytes = raw.len();
    let cluster_type = raw[0];
    match cluster_type {
        0 => {
            let mut buf = String::new();

            for i in 4..num_bytes {
                let byte = raw[i];
                if byte == 0 {
                    break;
                } else {
                    buf.push(byte as char);
                }
            }

            let name = buf.to_owned();
            let empty = raw[1] as usize;
            let damaged = raw[2] as usize;
            let headers = raw[3] as usize;

            Ok(Cluster::Root {
                name,
                empty,
                damaged,
                headers,
            })
        }
        1 => {
            let next_empty = raw[1] as usize;
            Ok(Cluster::Empty { next_empty })
        }
        2 => {
            let next_damaged = raw[1] as usize;
            Ok(Cluster::Damaged { next_damaged })
        }
        3 => {
            let mut buf = String::new();
            let mut content_start = num_bytes;

            for i in 3..num_bytes {
                let byte = raw[i];
                if byte == 0 {
                    content_start = i + 1;
                    break;
                } else {
                    buf.push(byte as char);
                }
            }

            let name = buf.to_owned();
            buf.clear();

            for i in content_start..num_bytes {
                let byte = raw[i];
                if byte == 0 {
                    break;
                } else {
                    buf.push(byte as char);
                }
            }

            let content = buf.to_owned();
            let next_header = raw[1] as usize;
            let next_data = raw[2] as usize;

            Ok(Cluster::FileHeader {
                name,
                content,
                next_header,
                next_data,
            })
        }
        4 => {
            let mut buf = String::new();

            for i in 2..num_bytes {
                let byte = raw[i];
                if byte == 0 {
                    break;
                } else {
                    buf.push(byte as char);
                }
            }

            let content = buf.to_owned();
            let next_data = raw[1] as usize;

            Ok(Cluster::FileData { content, next_data })
        }
        _ => Err("invalid cluster"),
    }
}

fn print_disk(clusters: &[Cluster]) {
    // Assumes disk has 64 columns
    println!("XX:                1               2               3");
    println!("XX:0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF");
    for (i, cluster) in clusters.iter().enumerate() {
        print!("{:02X}:", i);
        match cluster {
            Cluster::Root {
                name,
                empty,
                damaged,
                headers,
            } => println!(
                "0{:02X}{:02X}{:02X}{}0",
                empty,
                damaged,
                headers,
                pad(&hex::encode_upper(name), "0", 56)
            ),
            Cluster::Empty { next_empty } => println!("1{:02X}{}0", next_empty, pad("", "0", 60)),
            Cluster::Damaged { next_damaged } => {
                println!("2{:02X}{}0", next_damaged, pad("", "0", 60))
            }
            Cluster::FileHeader {
                name,
                content,
                next_header,
                next_data,
            } => println!(
                "3{:02X}{:02X}{}00{}0",
                next_header,
                next_data,
                &hex::encode_upper(name),
                pad(
                    &hex::encode_upper(content),
                    "0",
                    56 - &hex::encode_upper(name).len()
                )
            ),
            Cluster::FileData { content, next_data } => {
                println!(
                    "4{:02X}{}0",
                    next_data,
                    pad(&hex::encode_upper(content), "0", 60)
                )
            }
        };
    }
}

fn print_files(clusters: &[Cluster]) {
    for cluster in clusters {
        match cluster {
            Cluster::FileHeader { name, .. } => println!("{name}"),
            _ => (),
        }
    }
}

fn pad(s: &str, padding: &str, n: usize) -> String {
    let mut padded = String::new();
    padded.push_str(s);
    if s.len() >= n {
        return padded;
    }
    for _ in 0..(n - s.len()) {
        padded.push_str(padding);
    }

    padded
}
