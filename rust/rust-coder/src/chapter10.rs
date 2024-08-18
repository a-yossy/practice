use proconio::input;

pub fn main() {
  input! {
    x: i32,
    y: i32,
  }
  let rounded = x / y * y;
  assert_eq!(rounded % y, 0);
}
