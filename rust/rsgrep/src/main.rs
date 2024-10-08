use std::fs::read_to_string;
use rayon::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>
}

fn grep(state: &GrepArgs, content: String, file_name: &str) {
    for line in content.lines() {
        if line.contains(state.pattern.as_str()) {
            println!("{}: {}", file_name, line)
        }
    }
}

fn run(state: GrepArgs) {
    state
        .path
        .par_iter()
        .for_each(|file| match read_to_string(file) {
            Ok(content) => grep(&state, content, file),
            Err(reason) => println!("{}", reason),
        })
}

fn main() {
    run(GrepArgs::from_args());
}
