use clap::Parser;
use findr::Args;

fn main() {
    if let Err(e) = findr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
