use proconio::input;

pub fn main() {
  let reference;
  {
    reference = &100;
  }
  assert_eq!(*reference, 100);
}
