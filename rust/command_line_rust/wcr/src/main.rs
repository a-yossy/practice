use clap::Parser;
use wcr::Args;

fn main() {
    if let Err(e) = wcr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
