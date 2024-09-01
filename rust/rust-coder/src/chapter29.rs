use proconio::input;

pub fn main() {
  let s: String;
  let s = String::new();
  let slice: &str = &s;
  let slice = "Hello";

  let string = "Hello".to_string();
  let string = String::from("Hello");

  let greeting = "Hello";
  let world = "world".to_string();
  println!("{} {}", greeting, world);

  let s = "Hello";
  for c in s.chars() {
    print!("{}", c);
  }
  let s = "Hello".to_string();
  for c in s.chars() {
    println!("{}", c);
  }

  let s: &str = "打打打打打打打打打打打打打打打";
  let da: char = '打';
  let tmp = '打';
  let tmp2 = "打";

  let s = "𠮷野家で𩸽";
  for c in s.bytes() {
    println!("{:x}", c);
  }

  let s = format!("{} {}", 10, 2.5);

  let c = b'A';
  println!("{:x}", c);

  let array = b"Hello";

  input! {
    c1: char,
    c2: char,
    c3: char
  }
  println!("{} {} {}", c1, c2, c3);
}
