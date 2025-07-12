use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(required(true))]
    text: Vec<String>,
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
