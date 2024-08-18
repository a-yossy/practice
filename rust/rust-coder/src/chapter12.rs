use proconio::input;

pub fn main() {
  let array = [0, 10, 20, 30, 40, 50];
  let tuple = (0, 10, 20, 30, 40, 50);
  input! {
    index: usize,
  }
  let ans = array[index - 1];
  println!("{}", ans);
}
