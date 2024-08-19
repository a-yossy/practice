use proconio::input;

pub fn main() {
  let factor = 'input: loop {
      input! {
        x: i32,
      }
      for i in 2.. {
        if i * i > x {
          break;
        } else if x % i == 0 {
          break 'input i;
        }
      }
  };
  println!("{}", factor);
}
