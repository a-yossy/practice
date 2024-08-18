pub fn main() {
  let mut array = [10, 20, 30];
  for i in &mut array {
    *i += 1;
  }

  println!("{:?}", array);
}
