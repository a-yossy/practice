use clap::Parser;
use headr::Args;

fn main() {
    if let Err(e) = headr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
