fn main() {
  match 10 {
    0 => println!("0"),
    1..10 => println!("small number"),
    n => println!("big number: {}", n),
  }

  match (1.0, 1) {
    (0.0, 0) => println!("all zero"),
    (f, 0..10) => println!("float: {} with small number", f),
    _ => println!("other tuple"),
  }
}
