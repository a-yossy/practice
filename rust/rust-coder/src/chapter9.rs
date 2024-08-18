use proconio::input;

pub fn main() {
  input! {
    x: i32,
  }
  let abs;
  abs = if x >= 0 { x } else { -x };
  println!("{}", abs);
}
