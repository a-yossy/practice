use clap::Parser;
use lsr::Args;

fn main() {
    if let Err(e) = lsr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
