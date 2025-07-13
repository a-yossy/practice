use clap::Parser;
use uniqr::Args;

fn main() {
    if let Err(e) = uniqr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
