use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    if args.number_lines {
                        println!("{:>6}\t{line}", line_num + 1);
                    } else if args.number_nonblank_lines {
                        if !line.is_empty() {
                            prev_num += 1;
                            println!("{prev_num:>6}\t{line}");
                        } else {
                            println!();
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        };
    }
    Ok(())
}
