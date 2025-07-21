use clap::Parser;
use fortuner::Args;

fn main() {
    if let Err(e) = fortuner::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
