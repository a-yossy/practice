pub fn main() {
  println!("{2} {3} {0} {4} {1}", -2, 10, 2.4, 30, 4.5);
  println!("{0} {0} {1} {1}", -2, 10);
  println!("{hoge} {fuga} {hoge}", hoge = -2, fuga = 10);
  println!("{:06}", 79);

  let tuple = (10i32, 20i32, 30i32);
  println!("{:#?}", tuple);
}
