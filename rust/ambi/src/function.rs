fn add(x: isize, y: isize) -> isize {
  x + y
}
fn main() {
  println!("{}", add(1, 2));
  let f: fn(isize, isize) -> isize = add;
  let a = f(1, 2);
  println!("{}", a);
}
