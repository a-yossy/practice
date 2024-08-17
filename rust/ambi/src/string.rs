fn main() {
  let mut a: String = "abc".to_string();
  a += "def";
  println!("{}", a);

  let x = 1.0.to_string();
  println!("{}", x);

  a += x.as_str();
  println!("{}", a);
}
