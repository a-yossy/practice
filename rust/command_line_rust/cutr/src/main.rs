use clap::Parser;
use cutr::Args;

fn main() {
    if let Err(e) = cutr::run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
