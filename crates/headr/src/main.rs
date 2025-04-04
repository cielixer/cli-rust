use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;
use clap::{arg, Parser};

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Input file(s) to read
    #[arg(value_name = "FILE", default_value = "-", help = "Input file(s)")]
    files: Vec<String>,

    #[arg(
        short('n'), 
        long("lines"), 
        value_name = "LINES",
        conflicts_with = "bytes", 
        default_value = "10", 
        help = "Number of lines",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    #[arg(
        short('c'), 
        long("bytes"), 
        value_name = "BYTES",
        conflicts_with = "lines", 
        default_value=None, 
        help = "Number of bytes",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}

fn run(args: Args) -> Result<()> {
    let n_files = args.files.len();
    let n_lines = args.lines;

    for (idx, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {

                if n_files > 1 {
                    println!("{}==> {filename} <==", if idx > 0 { "\n" } else { "" });
                }

                if let Some(n_bytes) = args.bytes {
                    let mut buffer = vec![0; n_bytes as usize];
                    let r_bytes = file.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..r_bytes]));
                } else {
                    for _ in 0..n_lines {
                        let mut line = String::new();
                        file.read_line(&mut line)?;
                        print!("{line}");
                    }
                }
            },
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
    Ok(())
}