use std::io::{Read, BufRead, BufReader};

pub fn get_lines<T: Read>(f: T) -> Vec<String> {
  let f = BufReader::new(f);
  let mut lines_vec = Vec::new();

  for ll in f.lines() {
    lines_vec.push(ll.unwrap());
  }
  lines_vec
}
