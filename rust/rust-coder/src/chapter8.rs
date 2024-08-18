use proconio::input;

pub fn main() {
  input! {
    x: i32,
  }
  let abs;
  if x >= 0 {
    abs = x
  } else {
    abs = -x
  }
  println!("{}", abs);
}
