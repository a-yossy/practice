use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `uniq`
pub struct Args {
    /// Input file
    #[arg(default_value = "-")]
    in_file: String,

    /// Output file
    out_file: Option<String>,

    /// Show counts
    #[arg(short, long)]
    count: bool,
}

pub fn run(args: Args) -> Result<()> {
    println!("{:?}", args);
    Ok(())
}
