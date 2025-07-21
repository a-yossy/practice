use clap::Parser;
use commr::Args;

fn main() {
    if let Err(e) = commr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
