pub fn main() {
  println!("{}", std::cmp::max(2, 5));
  println!("{}", std::cmp::min(2, 5));

  let x = 10;
  let y = 10;
  let z = std::cmp::max(&x, &y);
  println!("&x: {:p}", &x);
  println!("&y: {:p}", &y);
  println!("z:  {:p}", z);
}
