use clap::Parser;
use calr::Args;

fn main() {
    if let Err(e) = calr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
