use std::sync::Arc;

pub fn main() {
  let mut s = "Hello, World!";
  println!("{}", s);

  let string = &s;
  println!("{}", s);
}
