use std::io;
use std::fs::File;

mod line_read;
mod tcp;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;

    let lines_vec = line_read::get_lines(f);

    println!("{:?}", lines_vec);
    Ok(())
}
