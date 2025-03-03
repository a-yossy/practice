use std::io::{self, BufReader};
use std::net::TcpListener;

use crate::line_read;

fn main() -> io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:3210")?;

  for stream in listener.incoming() {
    let lines_vec = line_read::get_lines(stream?);
    println!("{:?}", lines_vec);
  }

  Ok(())
}
