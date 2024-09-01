pub fn main() {
  println!("{:08b}", 25_u8);

  println!("{:08b}", -25_i8);
  println!("{:08b}", 231_u8);

  println!("{}", -20_i8 & -70_i8);
  println!("{}", -20_i8 | -70_i8);
  println!("{}", -20_i8 ^ -70_i8);
  println!("{}", !25_i8);
  println!("{}", 100_i8 << 2);
  println!("{}", 150_u8 >> 2);
  println!("{}", 50_i8 >> 2);
  println!("{}", -50_i8 >> 2);
}
