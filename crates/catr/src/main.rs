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
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short('n'), long("number"), conflicts_with = "number_nonblank_lines")]
    number_lines: bool,

    #[arg(short('b'), long("number-nonblank"), conflicts_with = "number_lines")]
    number_nonblank_lines: bool,
}

fn run(_args: Args) -> Result<()> {
    for filename in _args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                let mut line_number = 0;
                for line in file.lines() {
                    let mut line = line?;
                    
                    if _args.number_lines || (_args.number_nonblank_lines && !line.is_empty()) {
                        line_number += 1;
                        line = format!("{line_number:>6}\t{line}");
                    }
                    
                    println!("{line}");
                }
            }
        }
    }
    Ok(())
}

fn main(){
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}