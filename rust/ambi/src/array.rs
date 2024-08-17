fn main() {
  let a: [isize; 3] = [1, 2, 3];
  let b: &[isize] = &a;
  println!("{:?}", b);
  for elm in b {
    println!("{}", elm);
  }

  println!("{:?}", b[0]);
}
