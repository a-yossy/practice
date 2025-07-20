use clap::Parser;
use grepr::Args;

fn main() {
    if let Err(e) = grepr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
