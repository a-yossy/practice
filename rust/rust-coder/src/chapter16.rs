pub fn main() {
  let elements: [(i32, f64); 5] = [(6, 12.0), (7, 14.0), (8, 16.0), (15, 31.0), (16, 32.1)];
  for (number, weight) in &elements {
    println!("{:p}: {:p}", number, weight);
  }
}
