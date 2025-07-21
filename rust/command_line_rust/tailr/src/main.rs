use clap::Parser;
use tailr::Args;

fn main() {
    if let Err(e) = tailr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
