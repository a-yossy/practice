use catr::Args;
use clap::Parser;

fn main() {
    if let Err(e) = catr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
