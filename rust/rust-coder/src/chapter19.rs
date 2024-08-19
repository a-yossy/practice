pub fn main() {
  for i in 3.. {
      println!("{}", i);
      if i * i > 30 {
          break;
      }
  }
}
